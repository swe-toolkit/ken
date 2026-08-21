//! Inductive families — strict-positivity admission and the dependent
//! eliminator (`14-inductive.md`).
//!
//! Three loads:
//! - [`check_positivity`] — the strict-positivity check (`14 §8`), the
//!   **fixed** algorithm with `occurs`-guards on every position that could
//!   discard a subterm (application arguments `C u`, recursive-occurrence
//!   indices `D Δ_p t̄`, type parameters `X`). This is the soundness hole the
//!   Architect's review caught (`Bad3`/`Bad4`); the guards conservatively
//!   reject what K1 cannot prove strictly positive (`14 §8.4`).
//! - [`method_type`] — the dependent eliminator's per-constructor method type
//!   `Π Δₖ. Π (IHs). M t̄ₖ (cₖ p̄ Δₖ)`, computed from the family declaration and
//!   the concrete motive/params at a use site (`14 §3`, `14 §3.1`). W-style
//!   recursive args (`(b:B) → D Δ_p t̄[b]`) get a Π-abstracted IH
//!   `(b:B) → M t̄[b] (k b)` (K1.5).
//! - [`iota_reduct`] — the algorithmic ι-step `elim_D … (cₖ p̄ ā) ⇝ mₖ ā [IHs]`
//!   (`14 §7.3`, `14 §7.7`), capture-avoiding, with induction hypotheses on
//!   structurally smaller recursive arguments. W-style args produce a
//!   λ-abstracted IH `λb. elim_D … (k b)` (K1.5).
//!
//! **K1.5**: W-style (Π-bound) recursive arguments `(b:B) → D Δ_p t̄[b]` are
//! now **admitted** (`14 §2.1`, `14 §8.4`). The separate blanket gate
//! `check_no_pi_bound_recursive` is retired; strict positivity (`14 §8.2`) is
//! the sole structural admission test. The eliminator and ι handle the
//! Π-abstracted IH and the λ-threaded recursive call (`14 §3.1`, `14 §7.7`).
use std::collections::HashSet;

use crate::check::{infer_motive_level, Sort};
use crate::conv::{normalize, whnf};
use crate::env::{
    AllSupportSort, ConstructorDecl, Context, GlobalEnv, InductiveDecl, ParameterPolarity,
};
use crate::error::{KernelError, KernelResult};
use crate::subst::{apply_args, shift, subst_levels, subst_outer, subst_tel, weaken};
use crate::term::{GlobalId, Level, LevelVar, Term};
/// Does the inductive former `d` occur anywhere in `t` (syntactic sub-term)?
/// Used by the positivity guards (`14 §8`). de Bruijn indices make this
/// unambiguous: a former is a `Term::IndFormer { id, .. }` node.
pub fn occurs(d: GlobalId, t: &Term) -> bool {
    match t {
        Term::IndFormer { id, .. } => *id == d,
        _ => t.children().iter().any(|c| occurs(d, c)),
    }
}

/// Does `d` occur syntactically in `t` or in a transitively unfolded
/// transparent definition? The global environment is acyclic, so this closure
/// terminates.
fn occurs_delta(env: &GlobalEnv, d: GlobalId, t: &Term) -> bool {
    fn go(env: &GlobalEnv, d: GlobalId, t: &Term, seen: &mut HashSet<GlobalId>) -> bool {
        match t {
            Term::IndFormer { id, .. } => *id == d,
            Term::Const { id, .. } => {
                if !seen.insert(*id) {
                    return false;
                }
                env.transparent_body(*id)
                    .is_some_and(|(_, body)| go(env, d, &body, seen))
            }
            _ => t.children().iter().any(|child| go(env, d, child, seen)),
        }
    }

    go(env, d, t, &mut HashSet::new())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pol {
    Plus,
    Minus,
    Unknown,
}

impl Pol {
    fn flip(self) -> Pol {
        match self {
            Pol::Plus => Pol::Minus,
            Pol::Minus => Pol::Plus,
            Pol::Unknown => Pol::Unknown,
        }
    }
}

/// Peel a left-nested `App` spine into `(head, args)` left-to-right.
pub fn peel_app(t: &Term) -> (Term, Vec<Term>) {
    let mut args = Vec::new();
    let mut cur = t.clone();
    while let Term::App(f, a) = cur {
        args.push((*a).clone());
        cur = (*f).clone();
    }
    args.reverse();
    (cur, args)
}

/// Peel leading `Pi` binders into `(binder_domain_types, body)`.
pub fn peel_pi(t: &Term) -> (Vec<Term>, Term) {
    let mut doms = Vec::new();
    let mut cur = t.clone();
    while let Term::Pi(a, b) = cur {
        doms.push((*a).clone());
        cur = (*b).clone();
    }
    (doms, cur)
}

/// `check-pos-arg(D, pol, A)` — the strict-positivity judgment (`14 §8.2`).
///
/// Returns `true` if `A` is strictly positive in `D` at polarisation `pol`.
/// Every position that would discard a subterm without inspection is guarded
/// by an `occurs` check (the fixed algorithm): application arguments, the
/// indices of a recursive occurrence, and bare type parameters.
fn check_pos_arg(
    env: &GlobalEnv,
    d: GlobalId,
    pol: Pol,
    a: &Term,
    allow_terminal_supports: bool,
) -> bool {
    // WHNF exposes each head on demand; δ-aware guards close occurrences hidden
    // behind transparent definitions without materializing a full normal form.
    let head = whnf(env, &Context::new(), a);
    check_pos_arg_normalized(env, d, pol, &head, allow_terminal_supports)
}

fn check_pos_arg_normalized(
    env: &GlobalEnv,
    d: GlobalId,
    pol: Pol,
    a: &Term,
    allow_terminal_supports: bool,
) -> bool {
    match a {
        Term::Pi(dom, cod) => {
            check_pos_arg(env, d, pol.flip(), dom, allow_terminal_supports)
                && check_pos_arg(env, d, pol, cod, allow_terminal_supports)
        }
        Term::Sigma(dom, cod) => {
            check_pos_arg(env, d, pol, dom, allow_terminal_supports)
                && check_pos_arg(env, d, pol, cod, allow_terminal_supports)
        }
        Term::Lam(domain, body) => {
            !occurs_delta(env, d, domain)
                && check_pos_arg(env, d, pol, body, allow_terminal_supports)
        }
        Term::App(_, _) => {
            // `C u` (or `D Δ_p t̄` if the head is `D`).
            let (head, args) = peel_app(a);
            match head {
                Term::IndFormer { id, .. } if id == d => {
                    // Recursive occurrence `D Δ_p t̄`: positive polarity, and
                    // `D` must not occur in the (index) arguments.
                    pol == Pol::Plus && args.iter().all(|x| !occurs_delta(env, d, x))
                }
                Term::IndFormer { id, .. } => {
                    if args.iter().all(|argument| !occurs_delta(env, d, argument)) {
                        return true;
                    }
                    if env.is_terminal_support(id) && !allow_terminal_supports {
                        return false;
                    }
                    if pol != Pol::Plus {
                        return false;
                    }
                    let Some(former) = env.inductive(id) else {
                        return false;
                    };
                    args.iter().enumerate().all(|(position, argument)| {
                        if !occurs_delta(env, d, argument) {
                            return true;
                        }
                        position < former.params.len()
                            && former.parameter_polarities.get(position)
                                == Some(&ParameterPolarity::StrictlyPositive)
                            && check_pos_arg(env, d, Pol::Plus, argument, allow_terminal_supports)
                    })
                }
                Term::Const { .. } | Term::Constructor { .. } | Term::Var(_) => {
                    // An unresolved application head has no checked parameter
                    // polarity. Every argument therefore remains guarded.
                    args.iter().all(|argument| !occurs_delta(env, d, argument))
                }
                Term::Type(_) => {
                    // `Type ℓ` applied is ill-formed as a type; conservatively
                    // reject if `D` lurks anywhere.
                    args.is_empty() || !occurs_delta(env, d, a)
                }
                _ => {
                    // Pi/Sigma/Lam/... applied: ill-formed; conservative reject.
                    !occurs_delta(env, d, a)
                }
            }
        }
        Term::Type(_) => true, // `Type ℓ`; `D` is a type, not a level.
        Term::IndFormer { id, .. } if *id == d => {
            // Bare `D` (no arguments) — recursive occurrence with empty indices.
            pol == Pol::Plus
        }
        Term::IndFormer { .. } | Term::Const { .. } | Term::Constructor { .. } | Term::Var(_) => {
            // Bare `X` — a parameter or other type; reject if `D` occurs within.
            !occurs_delta(env, d, a)
        }
        // Anything else as a type is ill-formed; conservatively reject if D hides.
        _ => !occurs_delta(env, d, a),
    }
}

fn parameter_index(
    var: usize,
    local_depth: usize,
    prior_constructor_args: usize,
    parameter_count: usize,
) -> Option<usize> {
    let cutoff = local_depth.checked_add(prior_constructor_args)?;
    let relative = var.checked_sub(cutoff)?;
    if relative >= parameter_count {
        return None;
    }
    Some(parameter_count - 1 - relative)
}

struct ParameterPolarityDeriver<'a> {
    env: &'a GlobalEnv,
    d: GlobalId,
    parameter_count: usize,
    prior_constructor_args: usize,
    positive: &'a mut [bool],
    allow_terminal_supports: bool,
}

impl ParameterPolarityDeriver<'_> {
    fn visit(&mut self, local_depth: usize, pol: Pol, term: &Term) {
        match term {
            Term::Pi(dom, cod) => {
                self.visit(local_depth, pol.flip(), dom);
                self.visit(local_depth + 1, pol, cod);
            }
            Term::Sigma(dom, cod) => {
                self.visit(local_depth, pol, dom);
                self.visit(local_depth + 1, pol, cod);
            }
            Term::Lam(dom, body) => {
                self.visit(local_depth, Pol::Unknown, dom);
                self.visit(local_depth + 1, pol, body);
            }
            Term::Let { ty, val, body } => {
                self.visit(local_depth, Pol::Unknown, ty);
                self.visit(local_depth, Pol::Unknown, val);
                self.visit(local_depth + 1, Pol::Unknown, body);
            }
            Term::App(_, _) => {
                let (head, args) = peel_app(term);
                match head {
                    Term::IndFormer { id, .. } if id == self.d => {
                        for arg in &args {
                            self.visit(local_depth, pol, arg);
                        }
                    }
                    Term::IndFormer { id, .. } => {
                        let former = self.env.inductive(id).filter(|_| {
                            self.allow_terminal_supports || !self.env.is_terminal_support(id)
                        });
                        for (position, argument) in args.iter().enumerate() {
                            let argument_polarity = former
                                .filter(|declaration| position < declaration.params.len())
                                .and_then(|declaration| {
                                    declaration.parameter_polarities.get(position)
                                });
                            let nested_polarity = match argument_polarity {
                                Some(ParameterPolarity::StrictlyPositive) => pol,
                                Some(ParameterPolarity::NonPositive) | None => Pol::Unknown,
                            };
                            self.visit(local_depth, nested_polarity, argument);
                        }
                    }
                    Term::Var(_) => {
                        // A higher-kinded parameter used as the head of a type
                        // application occurs at the application's polarity.
                        // Its operands remain unclassified. Generated All
                        // predicates use exactly this shape, P x.
                        self.visit(local_depth, pol, &head);
                        for argument in &args {
                            self.visit(local_depth, Pol::Unknown, argument);
                        }
                    }
                    _ => {
                        // An unresolved head has no checked parameter polarity.
                        // Every argument therefore remains fail-closed.
                        self.visit(local_depth, Pol::Unknown, &head);
                        for arg in &args {
                            self.visit(local_depth, Pol::Unknown, arg);
                        }
                    }
                }
            }
            Term::Var(var) => {
                let parameter = parameter_index(
                    *var,
                    local_depth,
                    self.prior_constructor_args,
                    self.parameter_count,
                );
                if let Some(parameter) = parameter {
                    if pol != Pol::Plus {
                        self.positive[parameter] = false;
                    }
                }
            }
            Term::Type(_)
            | Term::Omega(_)
            | Term::IndFormer { .. }
            | Term::Const { .. }
            | Term::Constructor { .. } => {}
            _ => {
                // Pi, Sigma, Lam, and Let are Term's complete binder set and
                // have depth-aware arms above. Every remaining child stays at
                // the current depth.
                // Any parameter below a type form not covered by the D1a
                // grammar is unknown, hence not declared strictly positive.
                for child in term.children() {
                    self.visit(local_depth, Pol::Unknown, child);
                }
            }
        }
    }
}

/// Derive the fail-closed polarity record for an inductive family's parameters.
///
/// Applications of an already-admitted inductive former inherit only that
/// former's recorded strictly-positive parameter positions. Unresolved,
/// non-parameter, and non-positive positions remain fail-closed.
fn derive_parameter_polarities_inner(
    env: &GlobalEnv,
    ind: &InductiveDecl,
    allow_terminal_supports: bool,
) -> Vec<ParameterPolarity> {
    let mut positive = vec![true; ind.params.len()];

    // Dependent parameter types are not declared positive positions. Any
    // earlier parameter they mention is therefore conservatively non-positive.
    for (position, parameter_type) in ind.params.iter().enumerate() {
        ParameterPolarityDeriver {
            env,
            d: ind.id,
            parameter_count: position,
            prior_constructor_args: 0,
            positive: &mut positive[..position],
            allow_terminal_supports,
        }
        .visit(0, Pol::Unknown, parameter_type);
    }

    // Inductive indices likewise carry no declared parameter polarity. The
    // preceding indices occupy ordinary outer-context slots.
    for (position, index_type) in ind.indices.iter().enumerate() {
        ParameterPolarityDeriver {
            env,
            d: ind.id,
            parameter_count: ind.params.len(),
            prior_constructor_args: position,
            positive: &mut positive,
            allow_terminal_supports,
        }
        .visit(0, Pol::Unknown, index_type);
    }

    for constructor in &ind.constructors {
        for (argument, term) in constructor.args.iter().enumerate() {
            ParameterPolarityDeriver {
                env,
                d: ind.id,
                parameter_count: ind.params.len(),
                prior_constructor_args: argument,
                positive: &mut positive,
                allow_terminal_supports,
            }
            .visit(0, Pol::Plus, term);
        }
        // Target indices are values, not declared positive parameter
        // positions. They are scoped under the complete constructor telescope.
        for target_index in &constructor.target_indices {
            ParameterPolarityDeriver {
                env,
                d: ind.id,
                parameter_count: ind.params.len(),
                prior_constructor_args: constructor.args.len(),
                positive: &mut positive,
                allow_terminal_supports,
            }
            .visit(0, Pol::Unknown, target_index);
        }
    }
    positive
        .into_iter()
        .map(|is_positive| {
            if is_positive {
                ParameterPolarity::StrictlyPositive
            } else {
                ParameterPolarity::NonPositive
            }
        })
        .collect()
}

pub fn derive_parameter_polarities(env: &GlobalEnv, ind: &InductiveDecl) -> Vec<ParameterPolarity> {
    derive_parameter_polarities_inner(env, ind, false)
}

pub(crate) fn derive_support_parameter_polarities(
    env: &GlobalEnv,
    ind: &InductiveDecl,
) -> Vec<ParameterPolarity> {
    derive_parameter_polarities_inner(env, ind, true)
}

/// Run the strict-positivity check on a family declaration (`14 §8`): every
/// constructor argument type must be strictly positive in `D`. The family's
/// own parameters, indices, and each constructor's result target indices are
/// also `occurs`-checked (K1 rejects `D` appearing in its own indices, `Bad4`,
/// and nested parameter occurrences).
pub fn check_positivity(env: &GlobalEnv, ind: &InductiveDecl) -> KernelResult<()> {
    check_positivity_inner(env, ind, false)
}

pub(crate) fn check_support_positivity(env: &GlobalEnv, ind: &InductiveDecl) -> KernelResult<()> {
    check_positivity_inner(env, ind, true)
}

fn check_positivity_inner(
    env: &GlobalEnv,
    ind: &InductiveDecl,
    allow_terminal_supports: bool,
) -> KernelResult<()> {
    let d = ind.id;
    if ind.parameter_polarities.len() != ind.params.len() {
        return Err(KernelError::PositivityViolation(
            "parameter polarity record does not match the parameter telescope".into(),
        ));
    }
    let derived = derive_parameter_polarities_inner(env, ind, allow_terminal_supports);
    if ind
        .parameter_polarities
        .iter()
        .zip(derived)
        .any(|(recorded, actual)| recorded != &actual)
    {
        return Err(KernelError::PositivityViolation(
            "recorded parameter polarity does not match the declaration".into(),
        ));
    }
    for p in &ind.params {
        if occurs_delta(env, d, p) {
            return Err(KernelError::PositivityViolation(
                "D occurs in its own parameter telescope".into(),
            ));
        }
    }
    for ix in &ind.indices {
        if occurs_delta(env, d, ix) {
            return Err(KernelError::PositivityViolation(
                "D occurs in its own index telescope".into(),
            ));
        }
    }
    for c in &ind.constructors {
        for (j, a) in c.args.iter().enumerate() {
            if !check_pos_arg(env, d, Pol::Plus, a, allow_terminal_supports) {
                return Err(KernelError::PositivityViolation(format!(
                    "non-strictly-positive occurrence of D in constructor {:?} arg {j}",
                    c.id
                )));
            }
        }
        for (j, ix) in c.target_indices.iter().enumerate() {
            if occurs_delta(env, d, ix) {
                return Err(KernelError::PositivityViolation(format!(
                    "D occurs in constructor {:?} target index {j}",
                    c.id
                )));
            }
        }
    }
    Ok(())
}

/// The recursive arguments of a constructor: `(arg_position, branching_tel,
/// index_exprs)` for each arg whose type peels to `(b₁:B₁)...(b_{nb}:B_{nb})
/// → D Δ_p t̄` (K1.5, `14 §2.1`).
///
/// - `branching_tel` — the leading Π-binder domains `[B₁, B₂[b₁], ...]`
///   (empty for a direct `D Δ_p t̄`); each `B_k` is in context
///   `[Δ_p, args_before_pos, b₁..b_{k-1}]`.
/// - `index_exprs` — the index expressions after the family's `m` params, in
///   context `[Δ_p, args_before_pos, b₁..b_{nb}]` (under the branching binders).
pub fn recursive_args(
    c: &ConstructorDecl,
    d: GlobalId,
    m: usize,
) -> Vec<(usize, Vec<Term>, Vec<Term>)> {
    let mut out = Vec::new();
    for (j, a) in c.args.iter().enumerate() {
        let (pis, body) = peel_pi(a);
        let (head, args) = peel_app(&body);
        if let Term::IndFormer { id, .. } = head {
            if id == d && args.len() >= m {
                out.push((j, pis, args[m..].to_vec()));
            }
        }
    }
    out
}

#[derive(Clone)]
enum CarrierShape {
    Direct,
    Pi {
        body: Box<CarrierShape>,
    },
    Sigma {
        domain: Option<Box<CarrierShape>>,
        codomain: Option<Box<CarrierShape>>,
    },
    Former {
        former: GlobalId,
        arguments: Vec<Option<Box<CarrierShape>>>,
    },
}

fn occurs_context_var(term: &Term, target: usize, depth: usize) -> bool {
    match term {
        Term::Var(index) => *index == target + depth,
        Term::Pi(domain, codomain)
        | Term::Lam(domain, codomain)
        | Term::Sigma(domain, codomain) => {
            occurs_context_var(domain, target, depth)
                || occurs_context_var(codomain, target, depth + 1)
        }
        Term::Let { ty, val, body } => {
            occurs_context_var(ty, target, depth)
                || occurs_context_var(val, target, depth)
                || occurs_context_var(body, target, depth + 1)
        }
        _ => term
            .children()
            .iter()
            .any(|child| occurs_context_var(child, target, depth)),
    }
}

fn derive_carrier_shape(
    env: &GlobalEnv,
    term: &Term,
    target: usize,
) -> KernelResult<Option<CarrierShape>> {
    if !occurs_context_var(term, target, 0) {
        return Ok(None);
    }
    if matches!(term, Term::Var(index) if *index == target) {
        return Ok(Some(CarrierShape::Direct));
    }
    match term {
        Term::Pi(_, _) => {
            let (domains, body) = peel_pi(term);
            // Function domains do not contain runtime carrier values. A
            // covariant parameter occurrence that is visible only through
            // an even number of domain flips contributes no evidence field;
            // only the codomain supplies Π-abstracted evidence.
            Ok(
                derive_carrier_shape(env, &body, target + domains.len())?.map(|body| {
                    CarrierShape::Pi {
                        body: Box::new(body),
                    }
                }),
            )
        }
        Term::Sigma(domain, codomain) => Ok(Some(CarrierShape::Sigma {
            domain: derive_carrier_shape(env, domain, target)?.map(Box::new),
            codomain: derive_carrier_shape(env, codomain, target + 1)?.map(Box::new),
        })),
        Term::App(_, _) | Term::IndFormer { .. } => {
            let (head, arguments) = peel_app(term);
            let Term::IndFormer { id, .. } = head else {
                return Err(unsupported_recursive_shape(
                    "carrier occurrence has an unresolved application head",
                ));
            };
            let former = env.inductive(id).ok_or_else(|| {
                unsupported_recursive_shape("carrier occurrence lost former metadata")
            })?;
            let mut shapes = Vec::with_capacity(arguments.len());
            for (position, argument) in arguments.iter().enumerate() {
                let shape = derive_carrier_shape(env, argument, target)?;
                if shape.is_some()
                    && (position >= former.params.len()
                        || former.parameter_polarities.get(position)
                            != Some(&ParameterPolarity::StrictlyPositive))
                {
                    return Err(unsupported_recursive_shape(
                        "carrier occurrence is not in a checked positive parameter",
                    ));
                }
                shapes.push(shape.map(Box::new));
            }
            if shapes.iter().all(Option::is_none) {
                Ok(None)
            } else {
                Ok(Some(CarrierShape::Former {
                    former: id,
                    arguments: shapes,
                }))
            }
        }
        _ => Err(unsupported_recursive_shape(
            "positive carrier occurs in an unsupported type form",
        )),
    }
}

fn support_application(
    support: GlobalId,
    host: &InductiveDecl,
    host_level_args: &[Level],
    leaf_level: Level,
    arguments: &[Term],
    predicate: Term,
    value: Term,
) -> Term {
    let mut result = Term::indformer(
        support,
        host_level_args
            .iter()
            .cloned()
            .chain(std::iter::once(leaf_level))
            .collect(),
    );
    for argument in arguments.iter().take(host.params.len()) {
        result = Term::app(result, argument.clone());
    }
    result = Term::app(result, predicate);
    for argument in arguments.iter().skip(host.params.len()) {
        result = Term::app(result, argument.clone());
    }
    Term::app(result, value)
}

fn pack_component_types(mut components: Vec<Term>) -> KernelResult<Term> {
    let Some(mut result) = components.pop() else {
        return Err(unsupported_recursive_shape(
            "carrier lift contains no evidence component",
        ));
    };
    while let Some(component) = components.pop() {
        result = Term::sigma(component, weaken(&result, 1));
    }
    Ok(result)
}

fn recursive_shape_sort(
    env: &GlobalEnv,
    shape: &RecursiveShape,
    leaf: &Sort,
) -> KernelResult<Sort> {
    match shape {
        RecursiveShape::Direct { .. } => Ok(leaf.clone()),
        RecursiveShape::Pi { domains, body } => {
            let inner = recursive_shape_sort(env, body, leaf)?;
            let level = domains
                .iter()
                .try_fold(inner.level().clone(), |level, domain| {
                    Ok::<_, KernelError>(
                        level.max(
                            crate::check::classify(env, &Context::new(), domain)?
                                .level()
                                .clone(),
                        ),
                    )
                })?
                .normalize();
            Ok(match inner {
                Sort::Type(_) => Sort::Type(level),
                Sort::Omega(_) => Sort::Omega(level),
            })
        }
        RecursiveShape::Sigma { domain, codomain } => {
            let sorts = domain
                .iter()
                .chain(codomain)
                .map(|shape| recursive_shape_sort(env, shape, leaf))
                .collect::<KernelResult<Vec<_>>>()?;
            let level = sorts
                .iter()
                .fold(Level::zero(), |level, sort| level.max(sort.level().clone()))
                .normalize();
            Ok(if sorts.iter().all(|sort| matches!(sort, Sort::Omega(_))) {
                Sort::Omega(level)
            } else {
                Sort::Type(level)
            })
        }
        RecursiveShape::Former {
            former,
            level_args,
            arguments,
        } => {
            let leaf_level = arguments
                .iter()
                .filter_map(|argument| argument.shape.as_deref())
                .map(|shape| recursive_shape_sort(env, shape, leaf))
                .collect::<KernelResult<Vec<_>>>()?
                .into_iter()
                .fold(Level::zero(), |level, sort| level.max(sort.level().clone()));
            let host = env.inductive(*former).ok_or_else(|| {
                unsupported_recursive_shape("recursive sort lost host declaration")
            })?;
            let host_level = subst_levels(
                &Term::Type(host.level.clone()),
                &host.level_params,
                level_args,
            );
            let Term::Type(host_level) = host_level else {
                unreachable!("inductive level instantiation is a Type")
            };
            // Every declared-former All boundary carries topology in Type.
            // The caller combines this leaf-side level with the instantiated
            // host level while forming the concrete support application.
            Ok(Sort::Type(leaf_level.max(host_level).normalize()))
        }
    }
}

fn carrier_shape_sort(
    env: &GlobalEnv,
    shape: &CarrierShape,
    field_type: &Term,
    leaf: &Sort,
) -> KernelResult<Sort> {
    match shape {
        CarrierShape::Direct => Ok(leaf.clone()),
        CarrierShape::Pi { body } => {
            let (domains, codomain) = peel_pi(field_type);
            let inner = carrier_shape_sort(env, body, &codomain, leaf)?;
            let level = domains
                .iter()
                .try_fold(inner.level().clone(), |level, domain| {
                    Ok::<_, KernelError>(
                        level.max(
                            crate::check::classify(env, &Context::new(), domain)?
                                .level()
                                .clone(),
                        ),
                    )
                })?
                .normalize();
            Ok(match inner {
                Sort::Type(_) => Sort::Type(level),
                Sort::Omega(_) => Sort::Omega(level),
            })
        }
        CarrierShape::Sigma { domain, codomain } => {
            let Term::Sigma(first, second) = field_type else {
                return Err(unsupported_recursive_shape(
                    "carrier sort lost its Sigma type",
                ));
            };
            let mut sorts = Vec::new();
            if let Some(shape) = domain {
                sorts.push(carrier_shape_sort(env, shape, first, leaf)?);
            }
            if let Some(shape) = codomain {
                sorts.push(carrier_shape_sort(env, shape, second, leaf)?);
            }
            let level = sorts
                .iter()
                .fold(Level::zero(), |level, sort| level.max(sort.level().clone()))
                .normalize();
            Ok(if sorts.iter().all(|sort| matches!(sort, Sort::Omega(_))) {
                Sort::Omega(level)
            } else {
                Sort::Type(level)
            })
        }
        CarrierShape::Former { former, arguments } => {
            let (head, actual_arguments) = peel_app(field_type);
            let Term::IndFormer { id, level_args } = head else {
                return Err(unsupported_recursive_shape("carrier sort lost its former"));
            };
            if id != *former {
                return Err(unsupported_recursive_shape("carrier sort former mismatch"));
            }
            let host = env
                .inductive(*former)
                .ok_or_else(|| unsupported_recursive_shape("carrier sort lost host declaration"))?;
            let nested_level = arguments
                .iter()
                .zip(actual_arguments)
                .filter_map(|(shape, argument)| shape.as_deref().map(|shape| (shape, argument)))
                .map(|(shape, argument)| carrier_shape_sort(env, shape, &argument, leaf))
                .collect::<KernelResult<Vec<_>>>()?
                .into_iter()
                .fold(Level::zero(), |level, sort| level.max(sort.level().clone()));
            let host_level = subst_levels(
                &Term::Type(host.level.clone()),
                &host.level_params,
                &level_args,
            );
            let Term::Type(host_level) = host_level else {
                unreachable!("inductive level instantiation is a Type")
            };
            Ok(Sort::Type(nested_level.max(host_level).normalize()))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn carrier_lift_type(
    env: &GlobalEnv,
    ctx: &Context,
    shape: &CarrierShape,
    field_type: &Term,
    value: &Term,
    predicate: &Term,
    current_host: GlobalId,
    current_parameter: usize,
    current_sort: AllSupportSort,
    current_support: GlobalId,
    leaf_sort: &Sort,
) -> KernelResult<Term> {
    match shape {
        CarrierShape::Direct => Ok(Term::app(predicate.clone(), value.clone())),
        CarrierShape::Pi { body } => {
            let (domains, codomain) = peel_pi(field_type);
            let binder_count = domains.len();
            let mut applied = weaken(value, binder_count as i64);
            for binder in 0..binder_count {
                applied = Term::app(applied, Term::var(binder_count - 1 - binder));
            }
            let mut inner_ctx = ctx.clone();
            inner_ctx.extend_tel(&domains);
            let mut lifted = carrier_lift_type(
                env,
                &inner_ctx,
                body,
                &codomain,
                &applied,
                &weaken(predicate, binder_count as i64),
                current_host,
                current_parameter,
                current_sort,
                current_support,
                leaf_sort,
            )?;
            for domain in domains.into_iter().rev() {
                lifted = Term::pi(domain, lifted);
            }
            Ok(lifted)
        }
        CarrierShape::Sigma { domain, codomain } => {
            let Term::Sigma(first_type, second_family) = field_type else {
                return Err(unsupported_recursive_shape(
                    "carrier Sigma shape lost its Sigma type",
                ));
            };
            let first_value = Term::proj1(value.clone());
            let second_value = Term::proj2(value.clone());
            let mut components = Vec::new();
            if let Some(shape) = domain {
                components.push(carrier_lift_type(
                    env,
                    ctx,
                    shape,
                    first_type,
                    &first_value,
                    predicate,
                    current_host,
                    current_parameter,
                    current_sort,
                    current_support,
                    leaf_sort,
                )?);
            }
            if let Some(shape) = codomain {
                let second_type = crate::subst::subst0(second_family, &first_value);
                components.push(carrier_lift_type(
                    env,
                    ctx,
                    shape,
                    &second_type,
                    &second_value,
                    predicate,
                    current_host,
                    current_parameter,
                    current_sort,
                    current_support,
                    leaf_sort,
                )?);
            }
            pack_component_types(components)
        }
        CarrierShape::Former { former, arguments } => {
            let (head, actual_arguments) = peel_app(field_type);
            let Term::IndFormer {
                id,
                level_args: host_level_args,
            } = head
            else {
                return Err(unsupported_recursive_shape(
                    "carrier former shape lost its former",
                ));
            };
            if id != *former || actual_arguments.len() != arguments.len() {
                return Err(unsupported_recursive_shape(
                    "carrier former shape and instantiated type disagree",
                ));
            }
            let host = env
                .inductive(*former)
                .ok_or_else(|| unsupported_recursive_shape("carrier lift lost host declaration"))?;
            let mut components = Vec::new();
            for (parameter, shape) in arguments.iter().enumerate() {
                let Some(shape) = shape.as_deref() else {
                    continue;
                };
                let argument_type = actual_arguments[parameter].clone();
                let mut predicate_ctx = ctx.clone();
                predicate_ctx.push(argument_type.clone());
                let predicate_body = carrier_lift_type(
                    env,
                    &predicate_ctx,
                    shape,
                    &weaken(&argument_type, 1),
                    &Term::var(0),
                    &weaken(predicate, 1),
                    current_host,
                    current_parameter,
                    current_sort,
                    current_support,
                    leaf_sort,
                )?;
                let predicate_sort = carrier_shape_sort(env, shape, &argument_type, leaf_sort)?;
                let support_sort = match predicate_sort {
                    Sort::Type(_) => AllSupportSort::Type,
                    Sort::Omega(_) => AllSupportSort::Omega,
                };
                let support = if *former == current_host
                    && parameter == current_parameter
                    && support_sort == current_sort
                {
                    current_support
                } else {
                    env.all_support(*former, parameter, support_sort)
                        .ok_or_else(|| {
                            unsupported_recursive_shape(
                                "composed carrier lift has no generated support family",
                            )
                        })?
                };
                let lifted_predicate = if matches!(shape, CarrierShape::Direct) {
                    predicate.clone()
                } else {
                    Term::lam(argument_type, predicate_body)
                };
                components.push(support_application(
                    support,
                    host,
                    &host_level_args,
                    predicate_sort.level().clone(),
                    &actual_arguments,
                    lifted_predicate,
                    value.clone(),
                ));
            }
            pack_component_types(components)
        }
    }
}

/// Build one terminal source-indexed All declaration for a checked-positive
/// host carrier. Allocation and transactional publication remain in check.
pub(crate) fn build_all_support_decl(
    env: &GlobalEnv,
    host: &InductiveDecl,
    parameter: usize,
    sort: AllSupportSort,
    family: GlobalId,
    constructor_ids: &[GlobalId],
) -> KernelResult<InductiveDecl> {
    if constructor_ids.len() != host.constructors.len() {
        return Err(KernelError::IllFormedDecl(
            "generated All constructor-id arity mismatch".into(),
        ));
    }
    let leaf_var = LevelVar(
        host.level_params
            .iter()
            .map(|variable| variable.0)
            .max()
            .map_or(0, |maximum| maximum + 1),
    );
    let leaf_level = Level::Var(leaf_var);
    let mut level_params = host.level_params.clone();
    level_params.push(leaf_var);
    let mut params = host.params.clone();
    let carrier = Term::var(host.params.len() - 1 - parameter);
    let leaf_sort = match sort {
        AllSupportSort::Type => Term::Type(leaf_level.clone()),
        AllSupportSort::Omega => Term::Omega(leaf_level.clone()),
    };
    params.push(Term::pi(carrier, leaf_sort));
    let leaf_class = match sort {
        AllSupportSort::Type => Sort::Type(leaf_level.clone()),
        AllSupportSort::Omega => Sort::Omega(leaf_level.clone()),
    };
    let mut indices = host
        .indices
        .iter()
        .enumerate()
        .map(|(position, index)| shift(index, 1, position))
        .collect::<Vec<_>>();
    let host_level_args = host
        .level_params
        .iter()
        .map(|variable| Level::Var(*variable))
        .collect::<Vec<_>>();
    let mut source_type = Term::indformer(host.id, host_level_args.clone());
    let index_count = host.indices.len();
    for position in 0..host.params.len() {
        source_type = Term::app(
            source_type,
            Term::var(index_count + 1 + host.params.len() - 1 - position),
        );
    }
    for position in 0..index_count {
        source_type = Term::app(source_type, Term::var(index_count - 1 - position));
    }
    indices.push(source_type);

    let mut constructors = Vec::with_capacity(host.constructors.len());
    for (constructor_index, constructor) in host.constructors.iter().enumerate() {
        let field_count = constructor.args.len();
        let mut args = constructor
            .args
            .iter()
            .enumerate()
            .map(|(position, argument)| shift(argument, 1, position))
            .collect::<Vec<_>>();
        let mut ctor_ctx = Context::new();
        ctor_ctx.extend_tel(&params);
        ctor_ctx.extend_tel(&args);
        let mut evidence_count = 0usize;
        for (position, original_type) in constructor.args.iter().enumerate() {
            let target = position + host.params.len() - 1 - parameter;
            let Some(shape) = derive_carrier_shape(env, original_type, target)? else {
                continue;
            };
            let field_value = Term::var(evidence_count + field_count - 1 - position);
            let field_type = crate::check::infer(env, &ctor_ctx, &field_value)?;
            let predicate = Term::var(evidence_count + field_count);
            let evidence = carrier_lift_type(
                env,
                &ctor_ctx,
                &shape,
                &field_type,
                &field_value,
                &predicate,
                host.id,
                parameter,
                sort,
                family,
                &leaf_class,
            )?;
            args.push(evidence.clone());
            ctor_ctx.push(evidence);
            evidence_count += 1;
        }

        let mut target_indices = constructor
            .target_indices
            .iter()
            .map(|index| weaken(&shift(index, 1, field_count), evidence_count as i64))
            .collect::<Vec<_>>();
        let mut source = Term::constructor(constructor.id, host_level_args.clone());
        for position in 0..host.params.len() {
            source = Term::app(
                source,
                Term::var(evidence_count + field_count + 1 + host.params.len() - 1 - position),
            );
        }
        for position in 0..field_count {
            source = Term::app(
                source,
                Term::var(evidence_count + field_count - 1 - position),
            );
        }
        target_indices.push(source);
        constructors.push(ConstructorDecl {
            id: constructor_ids[constructor_index],
            args,
            target_indices,
            type_: Term::Type(Level::zero()),
            recursive_positions: Vec::new(),
        });
    }
    let mut declaration = InductiveDecl {
        id: family,
        level_params,
        params,
        parameter_polarities: Vec::new(),
        indices,
        level: leaf_level.max(host.level.clone()).normalize(),
        constructors,
        former_type: Term::Type(Level::zero()),
    };
    declaration.build_types();
    declaration.parameter_polarities = derive_support_parameter_polarities(env, &declaration);
    Ok(declaration)
}

/// Original host-field positions carrying one evidence argument in an aligned
/// generated `All` constructor. The returned order is the evidence-field order.
pub fn all_support_evidence_positions(
    env: &GlobalEnv,
    support: GlobalId,
    constructor: usize,
) -> KernelResult<Vec<usize>> {
    let (host, parameter, _) = env
        .all_support_origin(support)
        .ok_or_else(|| unsupported_recursive_shape("family is not a generated All support"))?;
    let declaration = env
        .inductive(host)
        .ok_or_else(|| unsupported_recursive_shape("All support lost its host declaration"))?;
    let host_constructor = declaration.constructors.get(constructor).ok_or_else(|| {
        unsupported_recursive_shape("All support constructor is not aligned with its host")
    })?;
    host_constructor
        .args
        .iter()
        .enumerate()
        .filter_map(|(position, argument)| {
            let target = position + declaration.params.len() - 1 - parameter;
            match derive_carrier_shape(env, argument, target) {
                Ok(Some(_)) => Some(Ok(position)),
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            }
        })
        .collect()
}

/// One constructor argument whose type contains structurally recursive
/// occurrences of the family being described.
///
/// [`method_type`] and [`iota_reduct`] consume this descriptor as one atomic
/// semantic unit: every generated structured binder has a matching lifted
/// ι-term.
#[derive(Clone, Debug)]
pub struct RecursiveArgumentShape {
    pub position: usize,
    pub shape: RecursiveShape,
}

/// Structural recipe for lifting a motive through one recursive argument.
///
/// The variants exhaust the positive recursive shapes in the core grammar:
/// direct occurrences, Π-bound/W-style occurrences, primitive dependent Σ,
/// and applications of an admitted former through checked positive parameter
/// positions. Field terms are normalized through the kernel's established
/// terminating δ+β semantics before this structural classification.
/// Retained [`Term`] and [`Level`] values are semantic payloads, not Rust
/// identity: normalization is not a canonical representative of conversion,
/// so the descriptor deliberately does not implement `PartialEq`/`Eq`.
/// A D-free field contributes no [`RecursiveArgumentShape`].
#[derive(Clone, Debug)]
pub enum RecursiveShape {
    /// `D Δ_p t̄`: one motive leaf, indexed by `t̄`.
    Direct { index_exprs: Vec<Term> },
    /// `(b₁:B₁)…(bₙ:Bₙ) → A`: preserve the branching telescope and lift `A`.
    Pi {
        domains: Vec<Term>,
        body: Box<RecursiveShape>,
    },
    /// `(x:A) × B`: preserve dependent Sigma topology. A D-free component is
    /// `None`; a recursive component retains its complete nested shape.
    Sigma {
        domain: Option<Box<RecursiveShape>>,
        codomain: Option<Box<RecursiveShape>>,
    },
    /// `F a₁…aₙ`: preserve the former and application spine. Each argument
    /// records its original term plus a lift only when its checked parameter
    /// position contains recursive content.
    Former {
        former: GlobalId,
        level_args: Vec<Level>,
        arguments: Vec<RecursiveFormerArgument>,
    },
}

/// One argument in a [`RecursiveShape::Former`] application spine.
#[derive(Clone, Debug)]
pub struct RecursiveFormerArgument {
    pub term: Term,
    pub shape: Option<Box<RecursiveShape>>,
}

impl RecursiveShape {
    /// Number of syntactic motive leaves represented by this recipe.
    ///
    /// Runtime multiplicity is supplied by the containing value's topology:
    /// e.g. the single leaf recipe under `List` occurs once per list element.
    pub fn leaf_count(&self) -> usize {
        match self {
            RecursiveShape::Direct { .. } => 1,
            RecursiveShape::Pi { body, .. } => body.leaf_count(),
            RecursiveShape::Sigma { domain, codomain } => domain
                .iter()
                .chain(codomain)
                .map(|shape| shape.leaf_count())
                .sum(),
            RecursiveShape::Former { arguments, .. } => arguments
                .iter()
                .filter_map(|argument| argument.shape.as_deref())
                .map(RecursiveShape::leaf_count)
                .sum(),
        }
    }

    /// Project the legacy direct/Π-bound class for consumers which have not
    /// moved to structured lifts. Structured Sigma/former recipes return
    /// `None`.
    pub fn as_legacy(&self) -> Option<(Vec<Term>, Vec<Term>)> {
        match self {
            RecursiveShape::Direct { index_exprs } => Some((Vec::new(), index_exprs.clone())),
            RecursiveShape::Pi { domains, body } => {
                let (mut inner_domains, index_exprs) = body.as_legacy()?;
                let mut all_domains = domains.clone();
                all_domains.append(&mut inner_domains);
                Some((all_domains, index_exprs))
            }
            RecursiveShape::Sigma { .. } | RecursiveShape::Former { .. } => None,
        }
    }
}

enum ShapeDerivation {
    DFree,
    Recursive(RecursiveShape),
}

fn unsupported_recursive_shape(message: impl Into<String>) -> KernelError {
    KernelError::PositivityViolation(message.into())
}

fn derive_recursive_shape(
    env: &GlobalEnv,
    term: &Term,
    d: GlobalId,
    parameter_count: usize,
) -> KernelResult<ShapeDerivation> {
    let head = whnf(env, &Context::new(), term);
    let term = &head;
    if !occurs_delta(env, d, term) {
        return Ok(ShapeDerivation::DFree);
    }

    match term {
        Term::Pi(_, _) => {
            let (domains, body) = peel_pi(term);
            if domains.iter().any(|domain| occurs_delta(env, d, domain)) {
                return Err(unsupported_recursive_shape(
                    "recursive occurrence in a Pi domain is not positive",
                ));
            }
            match derive_recursive_shape(env, &body, d, parameter_count)? {
                ShapeDerivation::DFree => Err(unsupported_recursive_shape(
                    "Pi field contains an unclassified recursive occurrence",
                )),
                ShapeDerivation::Recursive(body) => {
                    Ok(ShapeDerivation::Recursive(RecursiveShape::Pi {
                        domains,
                        body: Box::new(body),
                    }))
                }
            }
        }
        Term::Sigma(domain, codomain) => {
            let domain = match derive_recursive_shape(env, domain, d, parameter_count)? {
                ShapeDerivation::DFree => None,
                ShapeDerivation::Recursive(shape) => Some(Box::new(shape)),
            };
            let codomain = match derive_recursive_shape(env, codomain, d, parameter_count)? {
                ShapeDerivation::DFree => None,
                ShapeDerivation::Recursive(shape) => Some(Box::new(shape)),
            };
            Ok(ShapeDerivation::Recursive(RecursiveShape::Sigma {
                domain,
                codomain,
            }))
        }
        Term::App(_, _) | Term::IndFormer { .. } => {
            let (head, arguments) = peel_app(term);
            match head {
                Term::IndFormer { id, level_args: _ } if id == d => {
                    if arguments.len() < parameter_count
                        || arguments
                            .iter()
                            .any(|argument| occurs_delta(env, d, argument))
                    {
                        return Err(unsupported_recursive_shape(
                            "recursive family parameters or indices contain the family",
                        ));
                    }
                    Ok(ShapeDerivation::Recursive(RecursiveShape::Direct {
                        index_exprs: arguments[parameter_count..].to_vec(),
                    }))
                }
                Term::IndFormer { id, level_args } => {
                    if env.is_terminal_support(id) {
                        return Err(unsupported_recursive_shape(
                            "terminal All support cannot become an enclosing nested former",
                        ));
                    }
                    let former = env.inductive(id).ok_or_else(|| {
                        unsupported_recursive_shape(
                            "nested occurrence has no admitted former metadata",
                        )
                    })?;
                    if arguments.len() < former.params.len() {
                        return Err(unsupported_recursive_shape(
                            "nested former application is under-saturated",
                        ));
                    }

                    let mut shaped_arguments = Vec::with_capacity(arguments.len());
                    for (position, argument) in arguments.into_iter().enumerate() {
                        let shape = if occurs_delta(env, d, &argument) {
                            if position >= former.params.len()
                                || former.parameter_polarities.get(position)
                                    != Some(&ParameterPolarity::StrictlyPositive)
                            {
                                return Err(unsupported_recursive_shape(
                                    "recursive occurrence is not in a checked positive parameter",
                                ));
                            }
                            match derive_recursive_shape(env, &argument, d, parameter_count)? {
                                ShapeDerivation::DFree => {
                                    return Err(unsupported_recursive_shape(
                                        "positive parameter lost a recursive occurrence",
                                    ))
                                }
                                ShapeDerivation::Recursive(shape) => Some(Box::new(shape)),
                            }
                        } else {
                            None
                        };
                        shaped_arguments.push(RecursiveFormerArgument {
                            term: argument,
                            shape,
                        });
                    }
                    Ok(ShapeDerivation::Recursive(RecursiveShape::Former {
                        former: id,
                        level_args,
                        arguments: shaped_arguments,
                    }))
                }
                Term::Const { .. } => Err(unsupported_recursive_shape(
                    "recursive occurrence has an opaque or unresolved application head",
                )),
                _ => Err(unsupported_recursive_shape(
                    "recursive occurrence has an unresolved application head",
                )),
            }
        }
        _ => Err(unsupported_recursive_shape(
            "recursive occurrence is in an unsupported type form",
        )),
    }
}

/// Describe every positive recursive shape in a constructor telescope.
///
/// Unlike [`recursive_args`], this API represents primitive Sigma and
/// checked-positive former nesting for the atomic method/ι consumers.
pub fn recursive_shapes(
    env: &GlobalEnv,
    c: &ConstructorDecl,
    d: GlobalId,
    parameter_count: usize,
) -> KernelResult<Vec<RecursiveArgumentShape>> {
    let mut shapes = Vec::new();
    for (position, argument) in c.args.iter().enumerate() {
        if let ShapeDerivation::Recursive(shape) =
            derive_recursive_shape(env, argument, d, parameter_count)?
        {
            shapes.push(RecursiveArgumentShape { position, shape });
        }
    }
    Ok(shapes)
}

fn apply_motive(motive: &Term, indices: &[Term], value: Term) -> Term {
    let mut result = motive.clone();
    for index in indices {
        result = Term::app(result, index.clone());
    }
    Term::app(result, value)
}

fn guest_params_from_shape(
    field_type: &Term,
    shape: &RecursiveShape,
    parameter_count: usize,
) -> Option<Vec<Term>> {
    match shape {
        RecursiveShape::Direct { .. } => {
            let (_, arguments) = peel_app(field_type);
            (arguments.len() >= parameter_count)
                .then(|| arguments.into_iter().take(parameter_count).collect())
        }
        RecursiveShape::Pi { body, .. } => {
            let (_, codomain) = peel_pi(field_type);
            guest_params_from_shape(&codomain, body, parameter_count)
        }
        RecursiveShape::Sigma { domain, codomain } => {
            let Term::Sigma(first, second) = field_type else {
                return None;
            };
            domain
                .as_deref()
                .and_then(|shape| guest_params_from_shape(first, shape, parameter_count))
                .or_else(|| {
                    codomain
                        .as_deref()
                        .and_then(|shape| guest_params_from_shape(second, shape, parameter_count))
                })
        }
        RecursiveShape::Former { arguments, .. } => {
            let (_, actual_arguments) = peel_app(field_type);
            arguments
                .iter()
                .zip(actual_arguments)
                .find_map(|(argument, actual)| {
                    argument
                        .shape
                        .as_deref()
                        .and_then(|shape| guest_params_from_shape(&actual, shape, parameter_count))
                })
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn intrinsic_former_lift_type(
    env: &GlobalEnv,
    former: GlobalId,
    former_level_args: &[Level],
    arguments: &[RecursiveFormerArgument],
    value: &Term,
    motive: &Term,
    d: GlobalId,
    parameter_count: usize,
    guest_level_args: &[Level],
) -> KernelResult<Term> {
    let former_decl = env.inductive(former).ok_or_else(|| {
        unsupported_recursive_shape("intrinsic All lift lost its host declaration")
    })?;
    let guest_decl = env.inductive(d).ok_or_else(|| {
        unsupported_recursive_shape("intrinsic All lift lost its guest declaration")
    })?;
    if arguments.len() < former_decl.params.len() {
        return Err(unsupported_recursive_shape(
            "intrinsic All lift is under-saturated",
        ));
    }
    let guest_params = arguments
        .iter()
        .find_map(|argument| {
            argument
                .shape
                .as_deref()
                .and_then(|shape| guest_params_from_shape(&argument.term, shape, parameter_count))
        })
        .ok_or_else(|| unsupported_recursive_shape("intrinsic All lift has no guest path"))?;
    let leaf_sort = infer_motive_level(
        env,
        &Context::new(),
        guest_decl,
        guest_level_args,
        &guest_params,
        motive,
    )?;
    let actual_arguments = arguments
        .iter()
        .map(|argument| argument.term.clone())
        .collect::<Vec<_>>();
    let mut components = Vec::new();
    for (parameter, argument) in arguments.iter().enumerate() {
        let Some(shape) = argument.shape.as_deref() else {
            continue;
        };
        if parameter >= former_decl.params.len() {
            return Err(unsupported_recursive_shape(
                "intrinsic All path enters a host index",
            ));
        }
        let predicate_body = structured_lift_type(
            env,
            shape,
            &weaken(&argument.term, 1),
            &Term::var(0),
            &weaken(motive, 1),
            d,
            parameter_count,
            guest_level_args,
        )?;
        let predicate_sort = recursive_shape_sort(env, shape, &leaf_sort)?;
        let support_sort = match predicate_sort {
            Sort::Type(_) => AllSupportSort::Type,
            Sort::Omega(_) => AllSupportSort::Omega,
        };
        let support = env
            .all_support(former, parameter, support_sort)
            .ok_or_else(|| {
                unsupported_recursive_shape(
                    "intrinsic All lift has no generated support declaration",
                )
            })?;
        components.push(support_application(
            support,
            former_decl,
            former_level_args,
            predicate_sort.level().clone(),
            &actual_arguments,
            Term::lam(argument.term.clone(), predicate_body),
            value.clone(),
        ));
    }
    pack_component_types(components)
}

/// Expose exactly one Π layer at a time until the retained skeleton's group is
/// complete. A codomain may reveal the next layer only after δ/β reduction, so
/// a single WHNF followed by a syntactic [`peel_pi`] can under-count the group.
fn whnf_pi_spine(env: &GlobalEnv, field_type: &Term, expected_domains: usize) -> (Vec<Term>, Term) {
    let mut actual_domains = Vec::with_capacity(expected_domains);
    let mut actual_body = field_type.clone();
    while actual_domains.len() < expected_domains {
        match whnf(env, &Context::new(), &actual_body) {
            Term::Pi(domain, body) => {
                actual_domains.push(*domain);
                actual_body = *body;
            }
            exposed => {
                actual_body = exposed;
                break;
            }
        }
    }
    (actual_domains, actual_body)
}

/// Build `Lift_D(M, A, a)` from the D3a skeleton (`14 §3.2`).
///
/// `field_type`, `value`, and `motive` are already instantiated in one common
/// caller context.  Reading indices back from `field_type` is intentional: the
/// retained terms in [`RecursiveShape`] are semantic payloads rather than a
/// canonical representative of conversion.
fn structured_lift_type(
    env: &GlobalEnv,
    shape: &RecursiveShape,
    field_type: &Term,
    value: &Term,
    motive: &Term,
    d: GlobalId,
    parameter_count: usize,
    guest_level_args: &[Level],
) -> KernelResult<Term> {
    let field_type = whnf(env, &Context::new(), field_type);
    match shape {
        RecursiveShape::Direct { .. } => {
            let (head, arguments) = peel_app(&field_type);
            match head {
                Term::IndFormer { id, .. } if id == d && arguments.len() >= parameter_count => Ok(
                    apply_motive(motive, &arguments[parameter_count..], value.clone()),
                ),
                _ => Err(unsupported_recursive_shape(
                    "direct lift no longer has the recursive family at its head",
                )),
            }
        }
        RecursiveShape::Pi { domains, body } => {
            let (actual_domains, actual_body) = whnf_pi_spine(env, &field_type, domains.len());
            if actual_domains.len() != domains.len() {
                return Err(unsupported_recursive_shape(
                    "Pi lift skeleton and normalized field arity disagree",
                ));
            }
            let binder_count = actual_domains.len();
            let mut applied_value = weaken(value, binder_count as i64);
            for binder in 0..binder_count {
                applied_value = Term::app(applied_value, Term::var(binder_count - 1 - binder));
            }
            let mut lifted = structured_lift_type(
                env,
                body,
                &actual_body,
                &applied_value,
                &weaken(motive, binder_count as i64),
                d,
                parameter_count,
                guest_level_args,
            )?;
            for domain in actual_domains.into_iter().rev() {
                lifted = Term::pi(domain, lifted);
            }
            Ok(lifted)
        }
        RecursiveShape::Sigma { domain, codomain } => {
            let Term::Sigma(actual_domain, actual_codomain) = field_type else {
                return Err(unsupported_recursive_shape(
                    "Sigma lift skeleton no longer has a normalized Sigma field",
                ));
            };
            let first_value = normalize(env, &Context::new(), &Term::proj1(value.clone()));
            let second_value = normalize(env, &Context::new(), &Term::proj2(value.clone()));
            let first = domain
                .as_deref()
                .map(|shape| {
                    structured_lift_type(
                        env,
                        shape,
                        &actual_domain,
                        &first_value,
                        motive,
                        d,
                        parameter_count,
                        guest_level_args,
                    )
                })
                .transpose()?;
            let second_type = crate::subst::subst0(&actual_codomain, &first_value);
            let second = codomain
                .as_deref()
                .map(|shape| {
                    structured_lift_type(
                        env,
                        shape,
                        &second_type,
                        &second_value,
                        motive,
                        d,
                        parameter_count,
                        guest_level_args,
                    )
                })
                .transpose()?;
            match (first, second) {
                (Some(first), Some(second)) => Ok(Term::sigma(first, weaken(&second, 1))),
                (Some(only), None) | (None, Some(only)) => Ok(only),
                (None, None) => Err(unsupported_recursive_shape(
                    "Sigma lift skeleton contains no recursive component",
                )),
            }
        }
        RecursiveShape::Former {
            former,
            level_args: _,
            arguments,
        } => {
            let (actual_head, actual_arguments) = peel_app(&field_type);
            let Term::IndFormer {
                id: actual_former,
                level_args: actual_former_level_args,
            } = actual_head
            else {
                return Err(unsupported_recursive_shape(
                    "declared-former lift head is not an IndFormer",
                ));
            };
            if actual_former != *former || actual_arguments.len() != arguments.len() {
                return Err(unsupported_recursive_shape(
                    "declared-former lift skeleton and normalized field disagree",
                ));
            }
            let actual_arguments = arguments
                .iter()
                .zip(actual_arguments)
                .map(|(shape_argument, term)| RecursiveFormerArgument {
                    term,
                    shape: shape_argument.shape.clone(),
                })
                .collect::<Vec<_>>();
            intrinsic_former_lift_type(
                env,
                *former,
                &actual_former_level_args,
                &actual_arguments,
                value,
                motive,
                d,
                parameter_count,
                guest_level_args,
            )
        }
    }
}

fn structured_lift_term(
    env: &GlobalEnv,
    shape: &RecursiveShape,
    field_type: &Term,
    value: &Term,
    motive: &Term,
    methods: &[Term],
    d: GlobalId,
    parameter_count: usize,
    level_args: &[Level],
    params: &[Term],
) -> KernelResult<Term> {
    let field_type = whnf(env, &Context::new(), field_type);
    match shape {
        RecursiveShape::Direct { .. } => {
            let (head, arguments) = peel_app(&field_type);
            match head {
                Term::IndFormer { id, .. } if id == d && arguments.len() >= parameter_count => {
                    Ok(Term::Elim {
                        fam: d,
                        level_args: level_args.to_vec(),
                        params: params.to_vec(),
                        motive: Box::new(motive.clone()),
                        methods: methods.to_vec(),
                        indices: arguments[parameter_count..].to_vec(),
                        scrut: Box::new(value.clone()),
                    })
                }
                _ => Err(unsupported_recursive_shape(
                    "direct lifted term no longer has the recursive family at its head",
                )),
            }
        }
        RecursiveShape::Pi { domains, body } => {
            let (actual_domains, actual_body) = whnf_pi_spine(env, &field_type, domains.len());
            if actual_domains.len() != domains.len() {
                return Err(unsupported_recursive_shape(
                    "Pi lifted term skeleton and normalized field arity disagree",
                ));
            }
            let binder_count = actual_domains.len();
            let mut applied_value = weaken(value, binder_count as i64);
            for binder in 0..binder_count {
                applied_value = Term::app(applied_value, Term::var(binder_count - 1 - binder));
            }
            let mut lifted = structured_lift_term(
                env,
                body,
                &actual_body,
                &applied_value,
                &weaken(motive, binder_count as i64),
                &methods
                    .iter()
                    .map(|method| weaken(method, binder_count as i64))
                    .collect::<Vec<_>>(),
                d,
                parameter_count,
                level_args,
                &params
                    .iter()
                    .map(|param| weaken(param, binder_count as i64))
                    .collect::<Vec<_>>(),
            )?;
            for domain in actual_domains.into_iter().rev() {
                lifted = Term::lam(domain, lifted);
            }
            Ok(lifted)
        }
        RecursiveShape::Sigma { domain, codomain } => {
            let Term::Sigma(actual_domain, actual_codomain) = field_type else {
                return Err(unsupported_recursive_shape(
                    "Sigma lifted term skeleton no longer has a normalized Sigma field",
                ));
            };
            let first_value = normalize(env, &Context::new(), &Term::proj1(value.clone()));
            let second_value = normalize(env, &Context::new(), &Term::proj2(value.clone()));
            let first = domain
                .as_deref()
                .map(|shape| {
                    structured_lift_term(
                        env,
                        shape,
                        &actual_domain,
                        &first_value,
                        motive,
                        methods,
                        d,
                        parameter_count,
                        level_args,
                        params,
                    )
                })
                .transpose()?;
            let second_type = crate::subst::subst0(&actual_codomain, &first_value);
            let second = codomain
                .as_deref()
                .map(|shape| {
                    structured_lift_term(
                        env,
                        shape,
                        &second_type,
                        &second_value,
                        motive,
                        methods,
                        d,
                        parameter_count,
                        level_args,
                        params,
                    )
                })
                .transpose()?;
            match (first, second) {
                (Some(first), Some(second)) => Ok(Term::pair(first, second)),
                (Some(only), None) | (None, Some(only)) => Ok(only),
                (None, None) => Err(unsupported_recursive_shape(
                    "Sigma lifted term skeleton contains no recursive component",
                )),
            }
        }
        RecursiveShape::Former {
            former,
            level_args: _,
            arguments,
        } => {
            let (actual_head, actual_arguments) = peel_app(&field_type);
            let Term::IndFormer {
                id: actual_former,
                level_args: actual_former_level_args,
            } = actual_head
            else {
                return Err(unsupported_recursive_shape(
                    "declared-former lifted term head is not an IndFormer",
                ));
            };
            if actual_former != *former || actual_arguments.len() != arguments.len() {
                return Err(unsupported_recursive_shape(
                    "declared-former lifted term skeleton and normalized field disagree",
                ));
            }
            let actual_arguments = arguments
                .iter()
                .zip(actual_arguments)
                .map(|(shape_argument, term)| RecursiveFormerArgument {
                    term,
                    shape: shape_argument.shape.clone(),
                })
                .collect::<Vec<_>>();
            intrinsic_former_lift_term(
                env,
                *former,
                &actual_former_level_args,
                &actual_arguments,
                value,
                motive,
                methods,
                d,
                parameter_count,
                level_args,
                params,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn single_all_lift_term(
    env: &GlobalEnv,
    former: GlobalId,
    former_level_args: &[Level],
    arguments: &[RecursiveFormerArgument],
    parameter: usize,
    parameter_shape: &RecursiveShape,
    value: &Term,
    motive: &Term,
    methods: &[Term],
    d: GlobalId,
    parameter_count: usize,
    d_level_args: &[Level],
    d_params: &[Term],
    predicate_sort: &Sort,
) -> KernelResult<Term> {
    let former_decl = env
        .inductive(former)
        .ok_or_else(|| unsupported_recursive_shape("All lift term lost its host declaration"))?;
    let support_sort = match predicate_sort {
        Sort::Type(_) => AllSupportSort::Type,
        Sort::Omega(_) => AllSupportSort::Omega,
    };
    let support = env
        .all_support(former, parameter, support_sort)
        .ok_or_else(|| unsupported_recursive_shape("All lift term lost its support family"))?;
    let support_decl = env
        .inductive(support)
        .ok_or_else(|| unsupported_recursive_shape("All support is not an inductive"))?;
    let actual_arguments = arguments
        .iter()
        .map(|argument| argument.term.clone())
        .collect::<Vec<_>>();
    let predicate_body = structured_lift_type(
        env,
        parameter_shape,
        &weaken(&arguments[parameter].term, 1),
        &Term::var(0),
        &weaken(motive, 1),
        d,
        parameter_count,
        d_level_args,
    )?;
    let predicate = Term::lam(arguments[parameter].term.clone(), predicate_body);
    let mut source_type = Term::indformer(former, former_level_args.to_vec());
    for argument in &actual_arguments {
        source_type = Term::app(source_type, argument.clone());
    }
    let lifted_arguments = actual_arguments
        .iter()
        .map(|argument| weaken(argument, 1))
        .collect::<Vec<_>>();
    let body_type = support_application(
        support,
        former_decl,
        former_level_args,
        predicate_sort.level().clone(),
        &lifted_arguments,
        weaken(&predicate, 1),
        Term::var(0),
    );
    let host_level = subst_levels(
        &Term::Type(former_decl.level.clone()),
        &former_decl.level_params,
        former_level_args,
    );
    let Term::Type(host_level) = host_level else {
        unreachable!("inductive level instantiation is a Type")
    };
    let host_motive = Term::Ascript(
        Box::new(Term::lam(source_type.clone(), body_type)),
        Box::new(Term::pi(
            source_type.clone(),
            Term::Type(predicate_sort.level().clone().max(host_level).normalize()),
        )),
    );
    let source_params = actual_arguments[..former_decl.params.len()].to_vec();
    let mut host_methods = Vec::with_capacity(former_decl.constructors.len());
    for (constructor_index, constructor) in former_decl.constructors.iter().enumerate() {
        let host_recursive = recursive_shapes(env, constructor, former, former_decl.params.len())?;
        let host_method_type = method_type(
            env,
            former_decl,
            constructor_index,
            &host_motive,
            &source_params,
            former_level_args,
        )?;
        let (method_domains, _) = peel_pi(&host_method_type);
        let field_count = constructor.args.len();
        let ih_count = host_recursive.len();
        let binder_count = method_domains.len();
        if binder_count != field_count + ih_count {
            return Err(unsupported_recursive_shape(
                "All host method binder count disagrees with its shape",
            ));
        }
        let mut evidence = Vec::new();
        for position in 0..field_count {
            let carrier_target = position + former_decl.params.len() - 1 - parameter;
            if derive_carrier_shape(env, &constructor.args[position], carrier_target)?.is_none() {
                continue;
            }
            if let Some((ordinal, _)) = host_recursive
                .iter()
                .enumerate()
                .find(|(_, recursive)| recursive.position == position)
            {
                evidence.push(Term::var(ih_count - 1 - ordinal));
                continue;
            }
            let field_type = shift(
                &subst_levels(
                    &subst_outer(
                        &constructor.args[position],
                        former_decl.params.len(),
                        &source_params,
                        position,
                    ),
                    &former_decl.level_params,
                    former_level_args,
                ),
                (field_count - position + ih_count) as i64,
                0,
            );
            let ShapeDerivation::Recursive(shape) =
                derive_recursive_shape(env, &field_type, d, parameter_count)?
            else {
                return Err(unsupported_recursive_shape(
                    "All evidence field lost its guest recursive shape",
                ));
            };
            evidence.push(structured_lift_term(
                env,
                &shape,
                &field_type,
                &Term::var(ih_count + field_count - 1 - position),
                &weaken(motive, binder_count as i64),
                &methods
                    .iter()
                    .map(|method| weaken(method, binder_count as i64))
                    .collect::<Vec<_>>(),
                d,
                parameter_count,
                d_level_args,
                &d_params
                    .iter()
                    .map(|parameter| weaken(parameter, binder_count as i64))
                    .collect::<Vec<_>>(),
            )?);
        }
        let mut body = Term::constructor(
            support_decl.constructors[constructor_index].id,
            former_level_args
                .iter()
                .cloned()
                .chain(std::iter::once(predicate_sort.level().clone()))
                .collect(),
        );
        for source_parameter in &source_params {
            body = Term::app(body, weaken(source_parameter, binder_count as i64));
        }
        body = Term::app(body, weaken(&predicate, binder_count as i64));
        for position in 0..field_count {
            body = Term::app(body, Term::var(ih_count + field_count - 1 - position));
        }
        for component in evidence {
            body = Term::app(body, component);
        }
        for domain in method_domains.into_iter().rev() {
            body = Term::lam(domain, body);
        }
        crate::check::check(env, &Context::new(), &body, &host_method_type)?;
        host_methods.push(body);
    }
    Ok(Term::Elim {
        fam: former,
        level_args: former_level_args.to_vec(),
        params: source_params,
        motive: Box::new(host_motive),
        methods: host_methods,
        indices: actual_arguments[former_decl.params.len()..].to_vec(),
        scrut: Box::new(value.clone()),
    })
}

#[allow(clippy::too_many_arguments)]
fn intrinsic_former_lift_term(
    env: &GlobalEnv,
    former: GlobalId,
    former_level_args: &[Level],
    arguments: &[RecursiveFormerArgument],
    value: &Term,
    motive: &Term,
    methods: &[Term],
    d: GlobalId,
    parameter_count: usize,
    d_level_args: &[Level],
    d_params: &[Term],
) -> KernelResult<Term> {
    let guest = env
        .inductive(d)
        .ok_or_else(|| unsupported_recursive_shape("intrinsic All term lost guest declaration"))?;
    let guest_params = arguments
        .iter()
        .find_map(|argument| {
            argument
                .shape
                .as_deref()
                .and_then(|shape| guest_params_from_shape(&argument.term, shape, parameter_count))
        })
        .ok_or_else(|| unsupported_recursive_shape("intrinsic All term has no guest path"))?;
    let leaf_sort = infer_motive_level(
        env,
        &Context::new(),
        guest,
        d_level_args,
        &guest_params,
        motive,
    )?;
    let mut components = Vec::new();
    for (parameter, argument) in arguments.iter().enumerate() {
        let Some(shape) = argument.shape.as_deref() else {
            continue;
        };
        let predicate_sort = recursive_shape_sort(env, shape, &leaf_sort)?;
        components.push(single_all_lift_term(
            env,
            former,
            former_level_args,
            arguments,
            parameter,
            shape,
            value,
            motive,
            methods,
            d,
            parameter_count,
            d_level_args,
            d_params,
            &predicate_sort,
        )?);
    }
    match components.len() {
        0 => Err(unsupported_recursive_shape(
            "intrinsic All term has no evidence component",
        )),
        1 => Ok(components.pop().expect("length checked")),
        _ => Ok(components
            .into_iter()
            .rev()
            .reduce(|tail, head| Term::pair(head, tail))
            .expect("length checked")),
    }
}

/// The dependent eliminator's method type for constructor `k`:
/// `Π Δₖ. Π (IH₁…IH_p). M t̄ₖ (cₖ p̄ ā)` (`14 §3`, `14 §3.1`), in the
/// caller's context Γ.
///
/// W-style recursive args `(b:B) → D Δ_p t̄[b]` get a Π-abstracted IH
/// `(b:B) → M t̄[b] (k b)` (K1.5, `14 §3.1`).
///
/// `motive` (`M`) and `params` (`p̄`) are the concrete motive and param
/// instance at the use site (terms in Γ); `level_args` instantiate the
/// family's level parameters (used in the constructor reference).
pub fn method_type(
    env: &GlobalEnv,
    ind: &InductiveDecl,
    k: usize,
    motive: &Term,
    params: &[Term],
    level_args: &[Level],
) -> KernelResult<Term> {
    let c = &ind.constructors[k];
    let m = ind.params.len();
    let n = c.args.len();
    let rec = recursive_shapes(env, c, ind.id, m)?;
    let p = rec.len();

    // Conclusion `M t̄ₖ (cₖ p̄ ā')` in context [Γ, a₁'..aₙ', ih₁..ih_p]
    // (depth ctx_depth + n + p, but ctx_depth is implicit — we build relative
    // to Γ by weakening Γ-terms past the n+p new binders).
    let np = (n + p) as i64;
    let m_w = weaken(motive, np);
    let tgt: Vec<Term> = c
        .target_indices
        .iter()
        .map(|t| {
            weaken(
                &subst_levels(&subst_outer(t, m, params, n), &ind.level_params, level_args),
                p as i64,
            )
        })
        .collect();
    let mut capp = Term::Constructor {
        id: c.id,
        level_args: level_args.to_vec(),
    };
    for p in params {
        capp = Term::app(capp, weaken(p, np)); // p̄ weakened past args+IHs
    }
    for j in 0..n {
        // a_{j+1}' is at index (p + n - 1 - j) in [Γ, args, ihs].
        capp = Term::app(capp, Term::var(p + n - 1 - j));
    }
    let mut conclusion = m_w;
    for t in &tgt {
        conclusion = Term::app(conclusion, t.clone());
    }
    conclusion = Term::app(conclusion, capp);

    // Wrap IH binders innermost-first (ih_p … ih_1).
    // Each IH may be:
    //   - Direct (nb=0): `M idxs a_pos` — a plain type.
    //   - W-style (nb≥1): `Π(b₁:B₁)...(b_{nb}:B_{nb}). M idxs (a_pos b₁..b_{nb})`
    //     — a Π-type over the branching telescope (`14 §3.1`).
    let mut ty = conclusion;
    for j in (0..p).rev() {
        let pos = rec[j].position;
        let field_type = shift(
            &subst_levels(
                &subst_outer(&c.args[pos], m, params, pos),
                &ind.level_params,
                level_args,
            ),
            (n - pos + j) as i64,
            0,
        );
        let field_value = Term::var(n - 1 - pos + j);
        let ih_ty = structured_lift_type(
            env,
            &rec[j].shape,
            &field_type,
            &field_value,
            &weaken(motive, (n + j) as i64),
            ind.id,
            m,
            level_args,
        )?;
        ty = Term::pi(ih_ty, ty);
    }

    // Wrap arg binders innermost-first (aₙ' … a₁').
    for j in (0..n).rev() {
        let a_ty = subst_levels(
            &subst_outer(&c.args[j], m, params, j),
            &ind.level_params,
            level_args,
        ); // in [Γ, a₁'..a_j']
        ty = Term::pi(a_ty, ty);
    }
    Ok(ty)
}

/// Conservatively classify which recursive-IH binders occur in a method.
///
/// A method has the shape `λ args. λ ihs. body`. If WHNF does not expose that
/// complete lambda prefix, every IH is treated as used: over-building is only
/// a reduction loss, while skipping a live IH would be unsound.
fn method_ih_usage(
    env: &GlobalEnv,
    method: &Term,
    argument_count: usize,
    ih_count: usize,
) -> Vec<bool> {
    let mut cursor = whnf(env, &Context::new(), method);
    for _ in 0..argument_count {
        let Term::Lam(_, body) = cursor else {
            return vec![true; ih_count];
        };
        cursor = *body;
    }

    let mut usage = Vec::with_capacity(ih_count);
    for _ in 0..ih_count {
        let Term::Lam(_, body) = cursor else {
            return vec![true; ih_count];
        };
        usage.push(occurs_context_var(&body, 0, 0));
        cursor = *body;
    }
    usage
}

/// Remove only IH lambdas proven dead by [`method_ih_usage`]. The `subst0`
/// argument is never observed: the occurrence proof establishes that index 0
/// is absent, so this operation only closes the binder and shifts outer indices.
fn prune_unused_method_ihs(
    env: &GlobalEnv,
    method: &Term,
    argument_count: usize,
    ih_usage: &[bool],
) -> Option<Term> {
    fn go(term: Term, arguments: usize, usage: &[bool]) -> Option<Term> {
        if arguments > 0 {
            let Term::Lam(domain, body) = term else {
                return None;
            };
            return Some(Term::Lam(
                domain,
                Box::new(go(*body, arguments - 1, usage)?),
            ));
        }
        let Some((&used, remaining)) = usage.split_first() else {
            return Some(term);
        };
        let Term::Lam(domain, body) = term else {
            return None;
        };
        let body = go(*body, 0, remaining)?;
        if used {
            Some(Term::Lam(domain, Box::new(body)))
        } else {
            Some(crate::subst::subst0(&body, &Term::Type(Level::zero())))
        }
    }

    go(whnf(env, &Context::new(), method), argument_count, ih_usage)
}

/// The ι-reduct of an eliminator applied to a constructor-headed scrutinee
/// (`14 §7.3`): `elim_D p̄ M m̄ i̅ (cₖ p̄ ā) ⇝ mₖ ā [IHs]`.
///
/// `ctor_all_args` is the constructor's full argument spine `p̄ ++ ā` (params
/// then args), already peeled from the scrutinee. Returns the reduct, or an
/// error if the spine does not match the constructor's arity.
pub fn iota_reduct(
    env: &GlobalEnv,
    ind: &InductiveDecl,
    k: usize,
    level_args: &[Level],
    params: &[Term],
    motive: &Term,
    methods: &[Term],
    ctor_all_args: &[Term],
) -> KernelResult<Term> {
    let c = &ind.constructors[k];
    let m = ind.params.len();
    let n = c.args.len();
    // Arity guards: `raw_wf` checks only scoping for an `Elim`, but `whnf` calls
    // `iota_reduct` on any constructor-headed scrutinee. A raw-well-formed
    // `Elim` with too few params/methods/level-args would index out of bounds
    // here — the kernel contract is yes/no, never a crash (`18 §4`).
    if params.len() != m {
        return Err(KernelError::BadEliminator(format!(
            "expected {m} params, got {}",
            params.len()
        )));
    }
    if methods.len() != ind.constructors.len() {
        return Err(KernelError::BadEliminator(format!(
            "expected {} methods, got {}",
            ind.constructors.len(),
            methods.len()
        )));
    }
    if level_args.len() != ind.level_params.len() {
        return Err(KernelError::BadEliminator(format!(
            "expected {} level args, got {}",
            ind.level_params.len(),
            level_args.len()
        )));
    }
    if ctor_all_args.len() != m + n {
        return Err(KernelError::BadEliminator(format!(
            "constructor {:?} arity mismatch: expected {} args, got {}",
            c.id,
            m + n,
            ctor_all_args.len()
        )));
    }
    let ctor_args = &ctor_all_args[m..]; // ā (the actual constructor args)
    let method = &methods[k];

    let rec = recursive_shapes(env, c, ind.id, m)?;
    let mut ih_usage = method_ih_usage(env, method, n, rec.len());
    let reduced_method = if ih_usage.iter().any(|used| !used) {
        match prune_unused_method_ihs(env, method, n, &ih_usage) {
            Some(method) => method,
            None => {
                // Any shape uncertainty takes the conservative path.
                ih_usage.fill(true);
                method.clone()
            }
        }
    } else {
        method.clone()
    };
    // Induction hypotheses for each recursive arg (`14 §7.3`, `14 §7.7`):
    //   - Direct (nb=0):    `elim_D p̄ M m̄ idx(a_j) a_j`
    //   - W-style (nb≥1):  `λ(b₁:B₁)...(b_{nb}:B_{nb}). elim_D p̄ M m̄ idx(a_j b₁..b_{nb}) (a_j b₁..b_{nb})`
    let mut ihs: Vec<Term> = Vec::new();
    for (argument, used) in rec.iter().zip(ih_usage) {
        if !used {
            continue;
        }
        let pos = argument.position;
        let field_type = subst_levels(
            &subst_tel(
                &subst_outer(&c.args[pos], m, params, pos),
                &ctor_args[..pos],
            ),
            &ind.level_params,
            level_args,
        );
        ihs.push(structured_lift_term(
            env,
            &argument.shape,
            &field_type,
            &ctor_args[pos],
            motive,
            methods,
            ind.id,
            m,
            level_args,
            params,
        )?);
    }

    // `mₖ ā [IHs]` — method applied to the constructor args then the IHs.
    let mut full_args = ctor_args.to_vec();
    full_args.extend(ihs);
    Ok(apply_args(reduced_method, &full_args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::telescope_to_pi;
    use crate::term::{Level, LevelVar};

    fn d(id: u32) -> GlobalId {
        GlobalId(id)
    }

    #[test]
    fn occurs_finds_former() {
        // D applied to something containing D.
        let t = Term::app(Term::indformer(d(0), vec![]), Term::indformer(d(0), vec![]));
        assert!(occurs(d(0), &t));
        assert!(!occurs(d(1), &t));
    }

    #[test]
    fn method_ih_usage_tracks_each_binder_under_later_lambdas() {
        // λarg. λih1. λih2. λx. ih1: ih1 is index 2 in the final body.
        let ty = Term::Type(Level::zero());
        let method = Term::lam(
            ty.clone(),
            Term::lam(
                ty.clone(),
                Term::lam(ty.clone(), Term::lam(ty, Term::var(2))),
            ),
        );
        assert_eq!(
            method_ih_usage(&GlobalEnv::new(), &method, 1, 2),
            vec![true, false]
        );
    }

    #[test]
    fn method_ih_usage_treats_an_unexposed_method_as_all_live() {
        assert_eq!(
            method_ih_usage(&GlobalEnv::new(), &Term::var(0), 1, 2),
            vec![true, true]
        );
    }

    #[test]
    fn positivity_nat_accepted() {
        let env = GlobalEnv::new();
        // data Nat : Type 0 where zero : Nat ; suc : Nat → Nat
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![
                ConstructorDecl {
                    id: d(1),
                    args: vec![],
                    target_indices: vec![],
                    type_: Term::Type(Level::zero()),
                    recursive_positions: vec![],
                },
                ConstructorDecl {
                    id: d(2),
                    args: vec![Term::indformer(d(0), vec![])],
                    target_indices: vec![],
                    type_: Term::Type(Level::zero()),
                    recursive_positions: vec![],
                },
            ],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        assert!(check_positivity(&env, &ind).is_ok());
    }

    #[test]
    fn positivity_bad_rejected() {
        let env = GlobalEnv::new();
        // data Bad : Type 0 where mk : (Bad → Bool) → Bad
        let bool_ = Term::indformer(d(9), vec![]); // some other type `Bool`
        let arg = Term::pi(Term::indformer(d(0), vec![]), bool_); // Bad → Bool
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![ConstructorDecl {
                id: d(1),
                args: vec![arg],
                target_indices: vec![],
                type_: Term::Type(Level::zero()),
                recursive_positions: vec![],
            }],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        assert!(check_positivity(&env, &ind).is_err());
    }

    #[test]
    fn positivity_bad3_in_application_rejected() {
        let env = GlobalEnv::new();
        // data Bad3 : Type 0 where mk : Pair (Bad3 → Empty) Unit → Bad3
        // `Pair` is an inductive former (id 7); arg = Pair (Bad3→Empty) Unit.
        let empty = Term::indformer(d(8), vec![]);
        let bad3 = Term::indformer(d(0), vec![]);
        let unit = Term::indformer(d(6), vec![]);
        let pair_ty = Term::app(
            Term::app(Term::indformer(d(7), vec![]), Term::pi(bad3.clone(), empty)),
            unit,
        );
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![ConstructorDecl {
                id: d(1),
                args: vec![pair_ty],
                target_indices: vec![],
                type_: Term::Type(Level::zero()),
                recursive_positions: vec![],
            }],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        assert!(
            check_positivity(&env, &ind).is_err(),
            "Bad3 nested-negative-in-application must be rejected"
        );
    }

    #[test]
    fn positivity_bad4_in_own_indices_rejected() {
        let env = GlobalEnv::new();
        // data Bad4 : (Bad4 → Empty) → Type 0 where mk : Bad4 Empty
        let empty = Term::indformer(d(8), vec![]);
        let bad4 = Term::indformer(d(0), vec![]);
        let idx = Term::pi(bad4, empty); // Bad4 → Empty as an index
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![idx], // D in its own index telescope
            level: Level::zero(),
            constructors: vec![],
            former_type: Term::Type(Level::zero()),
        };
        let _ = telescope_to_pi; // keep import
        ind.build_types();
        assert!(
            check_positivity(&env, &ind).is_err(),
            "Bad4 D-in-own-indices must be rejected"
        );
    }

    #[test]
    fn w_style_pi_bound_admitted_in_k1p5() {
        let env = GlobalEnv::new();
        // data W : Type 0 where mk : (Nat → W) → W   (strictly positive W-style;
        // K1.5 admits it, `14 §2.1`, `14 §8.4`).
        let nat = Term::indformer(d(5), vec![]);
        let w = Term::indformer(d(0), vec![]);
        let arg = Term::pi(nat, w); // Nat → W
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![ConstructorDecl {
                id: d(1),
                args: vec![arg],
                target_indices: vec![],
                type_: Term::Type(Level::zero()),
                recursive_positions: vec![],
            }],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        assert!(
            check_positivity(&env, &ind).is_ok(),
            "W-style is strictly positive"
        );
        // K1.5: recursive_args now includes the W-style arg.
        let rec = recursive_args(&ind.constructors[0], d(0), 0);
        assert_eq!(rec.len(), 1);
        let (pos, branching_tel, _idxs) = &rec[0];
        assert_eq!(*pos, 0);
        assert_eq!(branching_tel.len(), 1, "one Π-binder (Nat)");
    }

    #[test]
    fn w_style_branching_domain_not_d_free_rejected() {
        let env = GlobalEnv::new();
        // data Bad5 : Type 0 where mk : (Bad5 → Bad5) → Bad5
        // The branching domain `Bad5` is not D-free: §8.2 checks the domain at
        // flipped (−) polarity and finds D there, so it rejects.
        // `14 §2.1` "B contains no occurrence of D"; conformance `wstyle-branching-
        // domain-not-d-free-rejected`. Soundness guard: gate-removal must not
        // relax the polarity check on the branching domain.
        let bad5 = Term::indformer(d(0), vec![]);
        // (Bad5 → Bad5) → Bad5: Pi(Pi(Bad5, Bad5), Bad5)
        let neg_arg = Term::pi(Term::pi(bad5.clone(), bad5.clone()), bad5);
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![ConstructorDecl {
                id: d(1),
                args: vec![neg_arg],
                target_indices: vec![],
                type_: Term::Type(Level::zero()),
                recursive_positions: vec![],
            }],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        assert!(
            check_positivity(&env, &ind).is_err(),
            "branching domain not D-free must be rejected by §8.2 polarity check"
        );
    }

    // --- B3a regression: iota_reduct must not panic on arity mismatch ---
    // (Architect review on dec_2hnhhdb7mrxze.) `raw_wf` checks only scoping for
    // an `Elim`; `whnf` calls `iota_reduct` on any constructor-headed scrutinee.
    // A raw-well-formed `Elim` with too few params/methods/level-args must
    // return `KernelError::BadEliminator`, never panic.

    fn nat_decl() -> InductiveDecl {
        // data Nat : Type 0 where zero : Nat ; suc : Nat → Nat
        let mut ind = InductiveDecl {
            id: d(0),
            level_params: vec![],
            params: vec![],
            parameter_polarities: vec![],
            indices: vec![],
            level: Level::zero(),
            constructors: vec![
                ConstructorDecl {
                    id: d(1),
                    args: vec![],
                    target_indices: vec![],
                    type_: Term::Type(Level::zero()),
                    recursive_positions: vec![],
                },
                ConstructorDecl {
                    id: d(2),
                    args: vec![Term::indformer(d(0), vec![])],
                    target_indices: vec![],
                    type_: Term::Type(Level::zero()),
                    recursive_positions: vec![],
                },
            ],
            former_type: Term::Type(Level::zero()),
        };
        ind.build_types();
        ind
    }

    #[test]
    fn iota_reduct_wrong_methods_arity_errors_not_panics() {
        let ind = nat_decl();
        // `zero` (k=0) has no args; ctor_all_args = [] (m=0, n=0). But supply
        // only ONE method (Nat has two constructors) → must error, not panic.
        let motive = Term::lam(Term::indformer(d(0), vec![]), Term::indformer(d(0), vec![]));
        let res = iota_reduct(
            &GlobalEnv::new(),
            &ind,
            0,
            &[],
            &[],
            &motive,
            std::slice::from_ref(&motive), // 1 method, expected 2
            &[],
        );
        assert!(matches!(res, Err(KernelError::BadEliminator(_))));
    }

    #[test]
    fn iota_reduct_wrong_ctor_arity_errors_not_panics() {
        let ind = nat_decl();
        // `suc` (k=1) expects 1 ctor arg; supply 0 → must error, not panic.
        let motive = Term::lam(Term::indformer(d(0), vec![]), Term::indformer(d(0), vec![]));
        let m1 = Term::lam(
            Term::indformer(d(0), vec![]),
            Term::lam(Term::indformer(d(0), vec![]), Term::indformer(d(0), vec![])),
        );
        let res = iota_reduct(
            &GlobalEnv::new(),
            &ind,
            1, // suc
            &[],
            &[],
            &motive,
            &[motive.clone(), m1],
            &[], // 0 ctor args, expected 1
        );
        assert!(matches!(res, Err(KernelError::BadEliminator(_))));
    }

    #[test]
    fn iota_reduct_wrong_level_arity_errors_not_panics() {
        // A level-polymorphic family: supply the wrong number of level args.
        let mut ind = nat_decl();
        ind.level_params = vec![LevelVar(0)]; // one level param
        let motive = Term::lam(Term::indformer(d(0), vec![]), Term::indformer(d(0), vec![]));
        let res = iota_reduct(
            &GlobalEnv::new(),
            &ind,
            0,
            &[Level::zero(), Level::zero()], // 2 level args, expected 1
            &[],
            &motive,
            &[motive.clone(), motive.clone()],
            &[],
        );
        assert!(matches!(res, Err(KernelError::BadEliminator(_))));
    }
}
