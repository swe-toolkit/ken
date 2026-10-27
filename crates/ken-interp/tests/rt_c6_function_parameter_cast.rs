//! RT-C6: spec 42 §3.3 and spec 16 §3.2, parameters-only inductive casts.
//! Compare constructor heads and selected ground applications, never closures.
use ken_elaborator::ElabEnv;
use ken_interp::{apply, eval, EvalStore, EvalVal};
use ken_kernel::{check, inductive::peel_app, whnf, Context, GlobalId, Term};

struct Fixture {
    elab: ElabEnv,
    store: EvalStore,
}

impl Fixture {
    fn new() -> Self {
        let mut elab = ElabEnv::new().expect("checked prelude");
        for declaration in [
            "data CFn (f : Nat -> Nat) : Type where { MkCFn : CFn f }",
            "data CPairFn (p : ((f : Nat -> Nat) × Nat)) : Type where { MkCPairFn : CPairFn p }",
            "data Box (n : Nat) : Type where { MkBox : Box n }",
            "data CDep (f : Nat -> Nat) : Type where { MkCDep : Box (f Zero) -> CDep f }",
            "fn idN (n : Nat) : Nat = n",
            "fn idf (n : Nat) : Nat = n",
            "fn succN (n : Nat) : Nat = Suc n",
            "fn addN (m : Nat) (n : Nat) : Nat = match m { Zero ↦ n; Suc k ↦ Suc (addN k n) }",
            "const idNop : Nat -> Nat = Axiom",
            "fn wrapOp (n : Nat) : Nat = idNop n",
            "fn tcast (ty : Type) (ty2 : Type) (e : Eq Type ty ty2) (t : ty) : ty2 = J (λx _. x) t e",
            "axiom axTwin : Eq Type (CFn idN) (CFn succN)",
            "axiom axOp : Eq Type (CFn idN) (CFn wrapOp)",
            "axiom axDep : Eq Type (CDep idN) (CDep succN)",
            "axiom axSuc : Eq Type (CFn idN) (CFn Suc)",
            "data CB (g : Bool -> Bool) : Type where { MkCB : CB g }",
            "data N2 : Type where { Z2 : N2; S2 : N2 -> N2; T2 : N2 -> N2 }",
            "data CN (g : N2 -> N2) : Type where { MkCN : CN g }",
            "axiom axST : Eq Type (CN S2) (CN T2)",
            "data P2 : Type where { MkP : Nat -> Nat -> P2 }",
            "data CP (g : Nat -> P2) : Type where { MkCP : CP g }",
            "axiom axP : Eq Type (CP (MkP Zero)) (CP (MkP (Suc Zero)))",
            "axiom axOpaque : Eq Type (CFn idN) (CFn idNop)",
            "data WT : Type where { WLeaf : WT; WNode : (Nat -> WT) -> WT }",
            "fn wtMotive (w : WT) : Type = Nat",
            "fn cfnMotive (g : Nat -> Nat) (c : CFn g) : Type = Nat",
        ] {
            elab.elaborate_decl(declaration)
                .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
        }
        Self {
            elab,
            store: EvalStore::new(),
        }
    }

    fn id(&self, name: &str) -> GlobalId {
        *self
            .elab
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("missing {name}"))
    }

    fn constant(&self, name: &str) -> Term {
        Term::const_(self.id(name), vec![])
    }

    fn zero(&self) -> Term {
        Term::constructor(self.id("Zero"), vec![])
    }

    fn one(&self) -> Term {
        Term::app(Term::constructor(self.id("Suc"), vec![]), self.zero())
    }

    fn identity(&self) -> Term {
        Term::lam(Term::indformer(self.id("Nat"), vec![]), Term::var(0))
    }

    fn via_idn(&self) -> Term {
        Term::lam(
            Term::indformer(self.id("Nat"), vec![]),
            Term::app(self.constant("idN"), Term::var(0)),
        )
    }

    fn family(&self, name: &str, param: Term) -> Term {
        Term::app(Term::indformer(self.id(name), vec![]), param)
    }

    fn ctor(&self, name: &str, param: Term) -> Term {
        Term::app(Term::constructor(self.id(name), vec![]), param)
    }

    fn cast(&self, source: Term, target: Term, payload: Term) -> Term {
        Term::Cast(
            Box::new(source.clone()),
            Box::new(target),
            Box::new(Term::Refl(Box::new(source))),
            Box::new(payload),
        )
    }

    fn via_j(&self, source: Term, target: Term, proof: Term, payload: Term) -> Term {
        [source, target, proof, payload]
            .into_iter()
            .fold(self.constant("tcast"), Term::app)
    }

    fn checked(&self, term: &Term, ty: &Term) {
        check(&self.elab.env, &Context::new(), term, ty)
            .unwrap_or_else(|error| panic!("cast must kernel-check: {error:?}"));
    }

    fn eval(&mut self, term: &Term) -> EvalVal {
        eval(&[], term, &self.elab.env, &mut self.store)
    }

    fn observe_head_and_function(
        &mut self,
        term: &Term,
        ty: &Term,
        constructor: &str,
        pair: bool,
        successor: bool,
    ) {
        self.checked(term, ty);
        let kernel = whnf(&self.elab.env, &Context::new(), term);
        assert!(
            matches!(peel_app(&kernel).0, Term::Constructor { id, .. } if id == self.id(constructor)),
            "kernel constructor head"
        );
        let EvalVal::Ctor { id, args, .. } = self.eval(term) else {
            panic!("checked cast must produce a constructor");
        };
        assert_eq!(id, self.id(constructor));
        assert_eq!(args.len(), 1);
        let function = if pair {
            let EvalVal::Pair { fst, snd, .. } = &args[0] else {
                panic!("nested function parameter must be a pair")
            };
            let one = self.one();
            assert_eq!(**snd, self.eval(&one));
            (**fst).clone()
        } else {
            args[0].clone()
        };
        assert!(matches!(function, EvalVal::Closure { .. }));
        for input in [self.zero(), self.one()] {
            let ground = self.eval(&input);
            let expected = if successor {
                let suc = Term::app(Term::constructor(self.id("Suc"), vec![]), input);
                self.eval(&suc)
            } else {
                ground.clone()
            };
            assert_eq!(
                apply(function.clone(), ground, &self.elab.env, &mut self.store),
                expected
            );
        }
    }
}

fn observe_refl(f: &mut Fixture, family: &str, ctor: &str, source: Term, target: Term, pair: bool) {
    let a = f.family(family, source.clone());
    let b = f.family(family, target);
    let term = f.cast(a, b.clone(), f.ctor(ctor, source));
    f.observe_head_and_function(&term, &b, ctor, pair, false);
}

fn observe_j(f: &mut Fixture, family: &str, ctor: &str, source: Term, target: Term, pair: bool) {
    let a = f.family(family, source.clone());
    let b = f.family(family, target);
    let proof = Term::Refl(Box::new(a.clone()));
    let term = f.via_j(a, b.clone(), proof, f.ctor(ctor, source));
    f.observe_head_and_function(&term, &b, ctor, pair, false);
}

/// Promise: durable invariant. MEASURED: a checked CFn refl cast yields the
/// same head and identity at two Nat inputs. CLAIMED: C6 computes row 5.
/// THE GAP: selected applications do not compare closures universally.
#[test]
fn closed_refl_cfn_computes() {
    let mut f = Fixture::new();
    let (source, target) = (f.identity(), f.via_idn());
    observe_refl(&mut f, "CFn", "MkCFn", source, target, false);
}

/// Promise: durable invariant. MEASURED: a checked pair-parameter refl cast
/// preserves its ground second component and two ground applications.
/// CLAIMED: C6 computes row 6. THE GAP: nested observations are selective.
#[test]
fn closed_refl_cpairfn_computes() {
    let mut f = Fixture::new();
    let (source, target) = (
        Term::pair(f.identity(), f.one()),
        Term::pair(f.via_idn(), f.one()),
    );
    observe_refl(&mut f, "CPairFn", "MkCPairFn", source, target, true);
}

/// Promise: durable invariant. MEASURED: the captured addN Zero parameter
/// evaluates through a checked cast and ground identity applications.
/// CLAIMED: the push does not require an empty function environment.
/// THE GAP: only this checked closed capture and two inputs are observed.
#[test]
fn closed_refl_captured_parameter_computes() {
    let mut f = Fixture::new();
    let (source, target) = (Term::app(f.constant("addN"), f.zero()), f.constant("idN"));
    observe_refl(&mut f, "CFn", "MkCFn", source, target, false);
}

/// Promise: durable invariant. MEASURED: J endpoint evaluation produces a
/// constructor and two ground identity applications for row 5. CLAIMED:
/// J reaches C6. THE GAP: no claim about all J motives or all closures.
#[test]
fn j_cfn_convertible_computes() {
    let mut f = Fixture::new();
    let (source, target) = (f.identity(), f.via_idn());
    observe_j(&mut f, "CFn", "MkCFn", source, target, false);
}

/// Promise: durable invariant. MEASURED: J preserves the pair's ground Nat
/// and the selected function applications in row 6. CLAIMED: nested
/// parameters reach C6. THE GAP: selected probes are not closure equality.
#[test]
fn j_cpairfn_convertible_computes() {
    let mut f = Fixture::new();
    let (source, target) = (
        Term::pair(f.identity(), f.one()),
        Term::pair(f.via_idn(), f.one()),
    );
    observe_j(&mut f, "CPairFn", "MkCPairFn", source, target, true);
}

/// Promise: durable invariant. MEASURED: a captured source parameter travels
/// through J to a CFn head and ground identity. CLAIMED: captured C6 route.
/// THE GAP: selected ground inputs do not inspect closure representation.
#[test]
fn j_captured_parameter_computes() {
    let mut f = Fixture::new();
    let (source, target) = (Term::app(f.constant("addN"), f.zero()), f.constant("idN"));
    observe_j(&mut f, "CFn", "MkCFn", source, target, false);
}

/// Promise: durable invariant. MEASURED: the J route at identical CFn idf
/// endpoints gives a constructor and identity at two ground inputs (F1).
/// CLAIMED: non-syntactic endpoint equality does not leave J Unknown.
/// THE GAP: this is a checked source row, not arbitrary closure equality.
#[test]
fn j_identical_idf_computes() {
    let mut f = Fixture::new();
    let idf = f.constant("idf");
    observe_j(&mut f, "CFn", "MkCFn", idf.clone(), idf, false);
}

/// Promise: durable invariant. MEASURED: a false-axiom J cast of a
/// field-free CFn yields the target's successor on Zero and One. CLAIMED:
/// parameters come from the target. THE GAP: the typed axiom is not a
/// consistent closed equality between these different functions.
#[test]
fn j_twin_uses_target_parameter() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.constant("idN"));
    let b = f.family("CFn", f.constant("succN"));
    let term = f.via_j(
        a,
        b.clone(),
        f.constant("axTwin"),
        f.ctor("MkCFn", f.constant("idN")),
    );
    f.observe_head_and_function(&term, &b, "MkCFn", false, true);
}

/// Promise: durable invariant. MEASURED: a checked dependent-field CDep
/// with convertible parameters rebuilds its head and retains MkBox Zero.
/// CLAIMED: the field-type gate admits its positive case. THE GAP: this
/// witness does not test a field under non-convertible parameter types.
#[test]
fn closed_refl_dependent_field_computes() {
    let mut f = Fixture::new();
    let a = f.family("CDep", f.identity());
    let b = f.family("CDep", f.via_idn());
    let box_zero = f.ctor("MkBox", f.zero());
    let value = Term::app(f.ctor("MkCDep", f.identity()), box_zero);
    let term = f.cast(a, b.clone(), value);
    f.checked(&term, &b);
    assert!(
        matches!(peel_app(&whnf(&f.elab.env, &Context::new(), &term)).0, Term::Constructor { id, .. } if id == f.id("MkCDep"))
    );
    let EvalVal::Ctor { id, args, .. } = f.eval(&term) else {
        panic!("convertible dependent field must rebuild")
    };
    assert_eq!(id, f.id("MkCDep"));
    assert_eq!(args.len(), 2);
    let expected_box = f.ctor("MkBox", f.zero());
    assert_eq!(args[1], f.eval(&expected_box));
}

/// Promise: durable fail-closed invariant. MEASURED: CDep with a field
/// indexed by a non-convertible parameter stays Unknown where kernel is
/// stuck. CLAIMED: the field-type gate blocks an unsound re-heading.
/// THE GAP: the axiom is a typed test premise, not consistent equality.
#[test]
fn j_dependent_field_twin_stays_unknown() {
    let mut f = Fixture::new();
    let a = f.family("CDep", f.constant("idN"));
    let b = f.family("CDep", f.constant("succN"));
    let field = f.ctor("MkBox", f.zero());
    let value = Term::app(f.ctor("MkCDep", f.constant("idN")), field);
    let term = f.via_j(a, b.clone(), f.constant("axDep"), value);
    f.checked(&term, &b);
    assert!(
        !matches!(
            peel_app(&whnf(&f.elab.env, &Context::new(), &term)).0,
            Term::Constructor { .. }
        ),
        "kernel must not re-head incompatible field"
    );
    assert_eq!(f.eval(&term), EvalVal::Unknown);
}

/// Promise: durable invariant. MEASURED: opaque conversion refuses a closed
/// refl at the kernel, yet a J equation with axiom re-heads a CFn parameter.
/// CLAIMED: this push is structural, not an opacity gate. THE GAP: the axiom
/// is not evidence that the opaque functions are actually equal.
#[test]
fn opaque_closed_refl_is_rejected_but_j_axiom_pushes() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.identity());
    let b = f.family(
        "CFn",
        Term::lam(
            Term::indformer(f.id("Nat"), vec![]),
            Term::app(f.constant("idNop"), Term::var(0)),
        ),
    );
    let invalid = f.cast(a.clone(), b.clone(), f.ctor("MkCFn", f.identity()));
    assert!(check(&f.elab.env, &Context::new(), &invalid, &b).is_err());

    let a = f.family("CFn", f.constant("idN"));
    let b = f.family("CFn", f.constant("wrapOp"));
    let term = f.via_j(
        a,
        b.clone(),
        f.constant("axOp"),
        f.ctor("MkCFn", f.constant("idN")),
    );
    f.checked(&term, &b);
    assert!(
        matches!(peel_app(&whnf(&f.elab.env, &Context::new(), &term)).0, Term::Constructor { id, .. } if id == f.id("MkCFn"))
    );
    assert!(matches!(f.eval(&term), EvalVal::Ctor { id, .. } if id == f.id("MkCFn")));
}

// RT-C6-NON-CLOSURE-FUNCTION-PARAMETER: every function-value representation.

fn kind(value: &EvalVal) -> &'static str {
    match value {
        EvalVal::Closure { .. } => "Closure",
        EvalVal::CtorPending { .. } => "CtorPending",
        EvalVal::IhClosure { .. } => "IhClosure",
        EvalVal::Unknown => "Unknown",
        EvalVal::Neutral => "Neutral",
        _ => "other",
    }
}

impl Fixture {
    fn suc(&self) -> Term {
        Term::constructor(self.id("Suc"), vec![])
    }

    fn lam_suc(&self) -> Term {
        Term::lam(
            Term::indformer(self.id("Nat"), vec![]),
            Term::app(self.suc(), Term::var(0)),
        )
    }

    fn con(&self, name: &str) -> Term {
        Term::constructor(self.id(name), vec![])
    }

    /// Kernel checks and re-heads; the interpreter re-heads at the target
    /// parameter, observed by applying it to `input`.
    fn pushed(
        &mut self,
        label: &str,
        term: &Term,
        ty: &Term,
        constructor: &str,
        input: Term,
        expected: Term,
    ) {
        self.checked(term, ty);
        let kernel = whnf(&self.elab.env, &Context::new(), term);
        assert!(
            matches!(peel_app(&kernel).0, Term::Constructor { id, .. } if id == self.id(constructor)),
            "{label}: kernel constructor head"
        );
        let value = self.eval(term);
        let EvalVal::Ctor { id, args, .. } = value else {
            panic!("{label}: interpreter gave {value:?}, not a constructor");
        };
        assert_eq!(id, self.id(constructor), "{label}: head");
        assert_eq!(args.len(), 1, "{label}: one parameter");
        let ground = self.eval(&input);
        let want = self.eval(&expected);
        assert_eq!(
            apply(args[0].clone(), ground, &self.elab.env, &mut self.store),
            want,
            "{label}: target parameter applied"
        );
    }
}

/// Promise: durable invariant. MEASURED: CFn Suc to CFn (λn. Suc n) by a
/// checked refl re-heads to MkCFn, and the target parameter maps Zero to
/// Suc Zero. CLAIMED: an unapplied constructor parameter is not refuted.
/// THE GAP: one constructor and one input.
#[test]
fn nonclosure_refl_ctor_to_closure_computes() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.suc());
    let b = f.family("CFn", f.lam_suc());
    let term = f.cast(a, b.clone(), f.ctor("MkCFn", f.suc()));
    let (zero, one) = (f.zero(), f.one());
    f.pushed("ctor->closure", &term, &b, "MkCFn", zero, one);
}

/// Promise: durable invariant. MEASURED: the reverse cast re-heads, and the
/// target Suc maps Zero to Suc Zero. CLAIMED: the mixed pair is tolerated
/// in both orders. THE GAP: one constructor and one input.
#[test]
fn nonclosure_refl_closure_to_ctor_computes() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.lam_suc());
    let b = f.family("CFn", f.suc());
    let term = f.cast(a, b.clone(), f.ctor("MkCFn", f.lam_suc()));
    let (zero, one) = (f.zero(), f.one());
    f.pushed("closure->ctor", &term, &b, "MkCFn", zero, one);
}

/// Promise: durable invariant. MEASURED: tcast (CFn idN) (CFn Suc) under
/// axSuc re-heads at the target Suc. CLAIMED: the J route reaches the same
/// push. THE GAP: the axiom is a typed premise, not a consistent equality.
#[test]
fn nonclosure_j_axiom_closure_to_ctor_computes() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.constant("idN"));
    let b = f.family("CFn", f.suc());
    let term = f.via_j(
        a,
        b.clone(),
        f.constant("axSuc"),
        f.ctor("MkCFn", f.constant("idN")),
    );
    let (zero, one) = (f.zero(), f.one());
    f.pushed("j idN->Suc", &term, &b, "MkCFn", zero, one);
}

/// Promise: durable invariant. MEASURED: the Suc to Suc and lambda to lambda
/// controls re-head on the base and with the change. CLAIMED: the change
/// leaves them computing. THE GAP: two controls, one input.
#[test]
fn nonclosure_same_representation_controls_compute() {
    let mut f = Fixture::new();
    for (label, p) in [("Suc->Suc", f.suc()), ("lam->lam", f.lam_suc())] {
        let a = f.family("CFn", p.clone());
        let term = f.cast(a.clone(), a.clone(), f.ctor("MkCFn", p));
        let (zero, one) = (f.zero(), f.one());
        f.pushed(label, &term, &a, "MkCFn", zero, one);
    }
}

/// Promise: durable invariant. MEASURED: a partially applied primitive
/// parameter (not_bool) and its eta-expansion re-head in both directions,
/// and the rebuilt parameter has the target's representation. CLAIMED: a
/// primitive CtorPending is a function value. THE GAP: the parameter is
/// not applied here, since not_bool on the Bool constructors is Neutral at
/// this layer on the base.
#[test]
fn nonclosure_refl_primitive_parameter_computes() {
    let mut f = Fixture::new();
    let bool_ty = Term::indformer(f.id("Bool"), vec![]);
    let not = f.constant("not_bool");
    let eta = Term::lam(bool_ty, Term::app(not.clone(), Term::var(0)));
    for (label, source, target) in [
        ("prim->closure", not.clone(), eta.clone()),
        ("closure->prim", eta.clone(), not.clone()),
    ] {
        let a = f.family("CB", source.clone());
        let b = f.family("CB", target.clone());
        let term = f.cast(a, b.clone(), f.ctor("MkCB", source));
        f.checked(&term, &b);
        let kernel = whnf(&f.elab.env, &Context::new(), &term);
        assert!(
            matches!(peel_app(&kernel).0, Term::Constructor { id, .. } if id == f.id("MkCB")),
            "{label}: kernel head"
        );
        let value = f.eval(&term);
        let EvalVal::Ctor { id, args, .. } = value else {
            panic!("{label}: interpreter gave {value:?}");
        };
        assert_eq!(id, f.id("MkCB"));
        let want = f.eval(&target);
        assert_eq!(
            kind(&args[0]),
            kind(&want),
            "{label}: parameter read from target"
        );
    }
}

/// Promise: durable invariant. MEASURED: under axST, CN S2 to CN T2
/// re-heads and the target parameter maps Z2 to T2 Z2. CLAIMED: distinct
/// constructor ids are not compared; the parameter comes from the target.
/// THE GAP: the axiom is a typed premise, not a consistent equality.
#[test]
fn nonclosure_j_distinct_constructor_ids_use_target() {
    let mut f = Fixture::new();
    let a = f.family("CN", f.con("S2"));
    let b = f.family("CN", f.con("T2"));
    let term = f.via_j(
        a,
        b.clone(),
        f.constant("axST"),
        f.ctor("MkCN", f.con("S2")),
    );
    let z2 = f.con("Z2");
    let expected = Term::app(f.con("T2"), z2.clone());
    f.pushed("j S2->T2", &term, &b, "MkCN", z2, expected);
}

/// Promise: durable invariant. MEASURED: under axP, CP (MkP Zero) to
/// CP (MkP One) re-heads and the target maps Zero to MkP One Zero. CLAIMED:
/// an equal-id CtorPending pair is tolerated, its arguments not compared.
/// THE GAP: the axiom is a typed premise, not a consistent equality.
#[test]
fn nonclosure_j_equal_id_distinct_args_use_target() {
    let mut f = Fixture::new();
    let p0 = Term::app(f.con("MkP"), f.zero());
    let p1 = Term::app(f.con("MkP"), f.one());
    let a = f.family("CP", p0.clone());
    let b = f.family("CP", p1.clone());
    let term = f.via_j(a, b.clone(), f.constant("axP"), f.ctor("MkCP", p0));
    let zero = f.zero();
    let expected = Term::app(p1, zero.clone());
    f.pushed("j MkP0->MkP1", &term, &b, "MkCP", zero, expected);
}

/// Promise: durable fail-closed invariant. MEASURED: an opaque function
/// parameter (idNop, Unknown at this layer) keeps the cast Unknown while
/// kernel whnf re-heads. CLAIMED: Unknown is not a function value. THE GAP:
/// Unknown is a permitted refusal here, not the kernel's answer.
#[test]
fn nonclosure_opaque_parameter_stays_unknown() {
    let mut f = Fixture::new();
    let a = f.family("CFn", f.constant("idN"));
    let b = f.family("CFn", f.constant("idNop"));
    let term = f.via_j(
        a,
        b.clone(),
        f.constant("axOpaque"),
        f.ctor("MkCFn", f.constant("idN")),
    );
    f.checked(&term, &b);
    assert!(
        matches!(peel_app(&whnf(&f.elab.env, &Context::new(), &term)).0, Term::Constructor { id, .. } if id == f.id("MkCFn")),
        "kernel re-heads the opaque parameter"
    );
    assert_eq!(f.eval(&term), EvalVal::Unknown);
}

/// Promise: durable invariant. MEASURED: inside a checked WT eliminator,
/// casting CFn ih to CFn (λn. ih n) re-heads, so the closed term reduces to
/// Suc Zero as in the kernel. CLAIMED: an unsaturated IhClosure is a
/// function value. THE GAP: one W-shaped family, one branch.
#[test]
fn nonclosure_ih_closure_parameter_computes() {
    let mut f = Fixture::new();
    let nat = Term::indformer(f.id("Nat"), vec![]);
    let wt = Term::indformer(f.id("WT"), vec![]);
    // In the WNode method, context [k, ih]: ih = Var(0), k = Var(1).
    let eta_ih = Term::lam(nat.clone(), Term::app(Term::var(1), Term::var(0)));
    let src = f.family("CFn", Term::var(0));
    let tgt = f.family("CFn", eta_ih.clone());
    let cast = Term::Cast(
        Box::new(src.clone()),
        Box::new(tgt.clone()),
        Box::new(Term::Refl(Box::new(src))),
        Box::new(f.ctor("MkCFn", Term::var(0))),
    );
    let body = Term::Elim {
        fam: f.id("CFn"),
        level_args: vec![],
        params: vec![eta_ih.clone()],
        motive: Box::new(Term::app(f.constant("cfnMotive"), eta_ih.clone())),
        methods: vec![f.one()],
        indices: vec![],
        scrut: Box::new(cast),
    };
    let node_method = Term::lam(
        Term::pi(nat.clone(), wt.clone()),
        Term::lam(Term::pi(nat.clone(), nat.clone()), body),
    );
    let scrut = Term::app(f.con("WNode"), Term::lam(nat.clone(), f.con("WLeaf")));
    let term = Term::Elim {
        fam: f.id("WT"),
        level_args: vec![],
        params: vec![],
        motive: Box::new(f.constant("wtMotive")),
        methods: vec![f.zero(), node_method],
        indices: vec![],
        scrut: Box::new(scrut),
    };
    f.checked(&term, &nat);
    let kernel = whnf(&f.elab.env, &Context::new(), &term);
    assert!(
        matches!(peel_app(&kernel).0, Term::Constructor { id, .. } if id == f.id("Suc")),
        "kernel reduces to Suc"
    );
    let value = f.eval(&term);
    let one = f.one();
    let expected = f.eval(&one);
    assert_eq!(value, expected);
}
