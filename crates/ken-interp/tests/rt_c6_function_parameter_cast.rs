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

/// Promise: durable invariant. MEASURED: checked closed casts at convertible
/// function-bearing parameters return constructor heads and identity at two
/// ground inputs. CLAIMED: the C6 parameters-only push computes. THE GAP:
/// selected applications are not a proof of arbitrary closure equivalence.
#[test]
fn closed_refl_pushes_convertible_function_parameters() {
    let mut f = Fixture::new();
    let src = f.identity();
    let dst = f.via_idn();
    for (family, constructor, source, target, pair) in [
        ("CFn", "MkCFn", src.clone(), dst.clone(), false),
        (
            "CPairFn",
            "MkCPairFn",
            Term::pair(src.clone(), f.one()),
            Term::pair(dst, f.one()),
            true,
        ),
        (
            "CFn",
            "MkCFn",
            Term::app(f.constant("addN"), f.zero()),
            f.constant("idN"),
            false,
        ),
    ] {
        let a = f.family(family, source.clone());
        let b = f.family(family, target);
        let term = f.cast(a, b.clone(), f.ctor(constructor, source));
        f.observe_head_and_function(&term, &b, constructor, pair, false);
    }
}

/// Promise: durable invariant. MEASURED: J's endpoint route computes for
/// convertible endpoints, including the exact identical-endpoint F1 row;
/// selected Nat applications remain ground identity. CLAIMED: both producer
/// sites of C6 reach the push. THE GAP: a finite probe does not enumerate
/// all possible source families or dependent fields.
#[test]
fn j_endpoints_push_convertible_and_identical_parameters() {
    let mut f = Fixture::new();
    let src = f.identity();
    let dst = f.via_idn();
    for (family, constructor, source, target, pair) in [
        ("CFn", "MkCFn", src.clone(), dst.clone(), false),
        (
            "CPairFn",
            "MkCPairFn",
            Term::pair(src, f.one()),
            Term::pair(dst, f.one()),
            true,
        ),
        (
            "CFn",
            "MkCFn",
            Term::app(f.constant("addN"), f.zero()),
            f.constant("idN"),
            false,
        ),
        ("CFn", "MkCFn", f.constant("idf"), f.constant("idf"), false),
    ] {
        let a = f.family(family, source.clone());
        let b = f.family(family, target);
        let proof = Term::Refl(Box::new(a.clone()));
        let term = f.via_j(a, b.clone(), proof, f.ctor(constructor, source));
        f.observe_head_and_function(&term, &b, constructor, pair, false);
    }
}

/// Promise: durable invariant. MEASURED: J with a false axiom re-heads a
/// field-free constructor at the target parameter, while a non-convertible
/// dependent field stays Unknown and kernel-stuck. CLAIMED: the target is
/// used but field types still guard the push. THE GAP: the axiom is a typed
/// test premise, not a consistent closed witness of unequal parameters.
#[test]
fn j_twin_uses_target_and_refuses_nonconvertible_field() {
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
