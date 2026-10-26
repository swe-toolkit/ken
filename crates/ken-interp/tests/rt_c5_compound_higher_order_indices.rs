//! RT-C5-COMPOUND-HIGHER-ORDER-INDICES: spec 42 §3.6 C5 regularity.
//! The function-bearing results are observed through checked constructor
//! heads and selected ground applications, never closure identity or layout.
use ken_elaborator::ElabEnv;
use ken_interp::{apply, eval, EvalStore, EvalVal};
use ken_kernel::{check, Context, GlobalId, Term};

struct Fixture {
    elab: ElabEnv,
    store: EvalStore,
}

impl Fixture {
    fn new() -> Self {
        let mut elab = ElabEnv::new().expect("checked prelude");
        for declaration in [
            "data CPair (p : ((n : Nat) × Nat)) : Type where { MkCPair : CPair p }",
            "data CFn (f : Nat -> Nat) : Type where { MkCFn : CFn f }",
            "data CPairFn (p : ((f : Nat -> Nat) × Nat)) : Type where { MkCPairFn : CPairFn p }",
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
            .unwrap_or_else(|| panic!("missing checked identity {name}"))
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

    fn indexed(&self, family: &str, index: Term) -> Term {
        Term::app(Term::indformer(self.id(family), vec![]), index)
    }

    fn intro(&self, constructor: &str, index: Term) -> Term {
        Term::app(Term::constructor(self.id(constructor), vec![]), index)
    }

    fn cast(&self, from: Term, to: Term, payload: Term) -> Term {
        Term::Cast(
            Box::new(from.clone()),
            Box::new(to),
            Box::new(Term::Refl(Box::new(from))),
            Box::new(payload),
        )
    }

    fn check(&self, cast: &Term, result_type: &Term) {
        check(&self.elab.env, &Context::new(), cast, result_type)
            .unwrap_or_else(|error| panic!("closed cast must kernel-check: {error:?}"));
    }

    fn eval(&mut self, term: &Term) -> EvalVal {
        eval(&[], term, &self.elab.env, &mut self.store)
    }

    fn observe_identity_index(&mut self, value: EvalVal, ctor: &str, under_pair: bool) {
        let EvalVal::Ctor { id, args, .. } = value else {
            panic!("checked indexed result must be a constructor: {value:?}")
        };
        assert_eq!(id, self.id(ctor));
        assert_eq!(args.len(), 1);
        let function = if under_pair {
            let EvalVal::Pair { fst, snd, .. } = &args[0] else {
                panic!("indexed result must carry a pair: {:?}", args[0]);
            };
            let one = self.one();
            assert_eq!(**snd, self.eval(&one), "ground second pair component");
            (**fst).clone()
        } else {
            args[0].clone()
        };
        assert!(matches!(&function, EvalVal::Closure { .. }));
        for input in [self.zero(), self.one()] {
            let ground = self.eval(&input);
            let observed = apply(
                function.clone(),
                ground.clone(),
                &self.elab.env,
                &mut self.store,
            );
            assert_eq!(
                observed, ground,
                "selected ground application of returned index"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: a closed, checked reflexive
/// cast at the comparable Σ(Nat,Nat) index returns its complete source value.
/// CLAIMED: the existing Pair comparison is not regressed. THE GAP: this
/// fixture covers relevant, constructor-valued pair components, not proof Σ.
#[test]
fn identical_endpoint_comparable_pair_still_returns_value() {
    let mut f = Fixture::new();
    let index = Term::pair(f.one(), f.zero());
    let ty = f.indexed("CPair", index.clone());
    let value = f.intro("MkCPair", index);
    let cast = f.cast(ty.clone(), ty.clone(), value.clone());
    f.check(&cast, &ty);
    let expected = f.eval(&value);
    assert!(matches!(&expected, EvalVal::Ctor { id, .. } if *id == f.id("MkCPair")));
    assert_eq!(f.eval(&cast), expected);
}

/// Promise class: durable invariant. MEASURED: the checked cast at exactly
/// identical CFn endpoints returns a constructor whose function index behaves
/// like identity on selected ground inputs. CLAIMED: C5 regularity no longer
/// fails merely because this index evaluates to a closure. THE GAP: selected
/// applications are observations, not equality of arbitrary closures.
#[test]
fn identical_endpoint_function_index_returns_value() {
    let mut f = Fixture::new();
    let index = f.identity();
    let ty = f.indexed("CFn", index.clone());
    let value = f.intro("MkCFn", index);
    let cast = f.cast(ty.clone(), ty.clone(), value.clone());
    f.check(&cast, &ty);
    let input = f.eval(&value);
    f.observe_identity_index(input, "MkCFn", false);
    let result = f.eval(&cast);
    f.observe_identity_index(result, "MkCFn", false);
}

/// Promise class: durable invariant. MEASURED: a checked function-valued
/// component nested in Pair crosses the same C5 boundary; the Nat component
/// and selected ground function applications survive. CLAIMED: Pair adds no
/// independent function-equality mechanism. THE GAP: callable behaviour is
/// sampled at two checked Nat inputs rather than compared by representation.
#[test]
fn identical_endpoint_function_inside_pair_returns_value() {
    let mut f = Fixture::new();
    let index = Term::pair(f.identity(), f.one());
    let ty = f.indexed("CPairFn", index.clone());
    let value = f.intro("MkCPairFn", index);
    let cast = f.cast(ty.clone(), ty.clone(), value.clone());
    f.check(&cast, &ty);
    let input = f.eval(&value);
    f.observe_identity_index(input, "MkCPairFn", true);
    let result = f.eval(&cast);
    f.observe_identity_index(result, "MkCPairFn", true);
}

/// Promise class: durable invariant. MEASURED: the kernel rejects a forged
/// refl between different function-parameter types, and direct evaluation of
/// that deliberately *unchecked* cast takes C6's parameters-only push
/// (`16 §3.2`): the constructor is re-headed at the target's parameter, read
/// from the target type and never compared with the source. CLAIMED: no
/// closure is compared. THE GAP: this row is not a well-typed term; function
/// difference is observed by application, not closure shape.
#[test]
fn differing_function_parameters_push_to_target() {
    let mut f = Fixture::new();
    let nat = Term::indformer(f.id("Nat"), vec![]);
    let identity = f.identity();
    let successor = Term::lam(
        nat,
        Term::app(Term::constructor(f.id("Suc"), vec![]), Term::var(0)),
    );
    let zero = f.zero();
    let zero_value = f.eval(&zero);
    let from_function = f.eval(&identity);
    let to_function = f.eval(&successor);
    let from_at_zero = apply(from_function, zero_value.clone(), &f.elab.env, &mut f.store);
    let to_at_zero = apply(to_function, zero_value.clone(), &f.elab.env, &mut f.store);
    assert_ne!(
        from_at_zero, to_at_zero,
        "functions differ on a ground Nat input"
    );
    let from = f.indexed("CFn", identity.clone());
    let to = f.indexed("CFn", successor);
    let value = f.intro("MkCFn", identity);
    let invalid_cast = f.cast(from, to.clone(), value);
    assert!(check(&f.elab.env, &Context::new(), &invalid_cast, &to).is_err());
    let EvalVal::Ctor { id, args, .. } = f.eval(&invalid_cast) else {
        panic!("the parameters-only push re-heads the constructor");
    };
    assert_eq!(id, f.id("MkCFn"));
    let pushed_at_zero = apply(args[0].clone(), zero_value, &f.elab.env, &mut f.store);
    assert_eq!(pushed_at_zero, to_at_zero, "the parameter is the target's");
}

// The new Unknown-operand guard has no naturally closed checked fixture here:
// kernel Term has no hole and the checked prelude supplies no bodyless value
// of these indexed types. Adding an Axiom solely to fabricate Unknown would
// change the trust premise. The existing unedited
// `unknown_proof_blocks_equal_inductive_type_app_cast` unit instead covers
// the value-level reducer, not this term-level guard; it is not misreported
// as reaching the new arm. This is the D0 ruling's no-constructible-row case.
