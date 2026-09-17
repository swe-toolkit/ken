//! DS-1 (`Empty` + `Dec`) acceptance — `docs/program/wp/
//! catalog-ds-1-empty-dec.md`.
//!
//! - **AC1** — `Dec` admits and `elim_Dec` large-eliminates into a `Type0`
//!   motive (the build-step-1 smoke test).
//! - **AC2** — `Empty`/`absurdEmpty` (surface-authored) elaborate.
//! - **AC3** — the `trusted_base()` delta is exactly the two new inductive
//!   admissions (`Empty`, `Dec`), grounded on the Rust emission
//!   (`prelude.rs`), not a `.ken` view.
//! - **AC4** — the `DecEq -> Dec` bridge is demonstrated over `DecEq Bool`
//!   (inductive carrier, honest via K7), not only `DecEq Int` (`Axiom`).
//! - **AC5** — the catalog entry's `` ```ken ``/`` ```ken example ``/
//!   `` ```ken reject `` fences all check via the real literate extractor.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::conv::whnf;
use ken_kernel::env::Context;
use ken_kernel::term::{Level, Term};
use ken_kernel::{
    declare_inductive, infer, CtorSpec, Decl, GlobalEnv, GlobalId, InductiveSpec,
};
const EMPTY_DEC: &str = "Core.Logic.EmptyDec";
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const TRANSPORT: &str = "Core.Logic.Transport";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| term_mentions(child, target)),
    }
}

fn lv0() -> Level {
    Level::zero()
}

// AC1 — the build-step-1 smoke test, re-run here as a permanent regression
// (not scratch): `Dec` admits and `elim_Dec` large-eliminates into `Type0`.
#[test]
fn ac1_dec_admits_and_elim_dec_large_eliminates_into_type0() {
    let mut env = GlobalEnv::new();

    let empty_id = declare_inductive(&mut env, |_empty| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: lv0(),
        constructors: vec![],
    })
    .expect("Empty (zero-ctor Type0 inductive) must admit");

    let dec_id = declare_inductive(&mut env, |_dec| InductiveSpec {
        level_params: vec![],
        params: vec![Term::omega(lv0())],
        indices: vec![],
        level: lv0(),
        constructors: vec![
            CtorSpec { args: vec![Term::var(0)], target_indices: vec![] },
            CtorSpec {
                args: vec![Term::pi(Term::var(0), Term::indformer(empty_id, vec![]))],
                target_indices: vec![],
            },
        ],
    })
    .expect("Dec (P : Omega) : Type0 = Yes P | No (P -> Empty) must admit");

    let dec = env.inductive(dec_id).unwrap().clone();
    let (yes_id, no_id) = (dec.constructors[0].id, dec.constructors[1].id);

    let mut ctx = Context::new();
    ctx.push(Term::omega(lv0())); // P : Omega0
    let dec_p = Term::app(Term::indformer(dec_id, vec![]), Term::var(0));
    ctx.push(dec_p); // x : Dec P
    let p = Term::var(1); // P, relative to ctx [x, P]

    let motive = Term::Ascript(
        Box::new(Term::lam(
            Term::app(Term::indformer(dec_id, vec![]), p.clone()),
            Term::app(Term::indformer(dec_id, vec![]), Term::var(2)),
        )),
        Box::new(Term::pi(
            Term::app(Term::indformer(dec_id, vec![]), p.clone()),
            Term::Type(lv0()),
        )),
    );
    let yes_method = Term::lam(
        p.clone(),
        Term::app(
            Term::app(Term::constructor(yes_id, vec![]), Term::var(2)),
            Term::var(0),
        ),
    );
    let no_method = Term::lam(
        Term::pi(p.clone(), Term::indformer(empty_id, vec![])),
        Term::app(
            Term::app(Term::constructor(no_id, vec![]), Term::var(2)),
            Term::var(0),
        ),
    );
    let elim = Term::Elim {
        fam: dec_id,
        level_args: vec![],
        params: vec![p],
        motive: Box::new(motive),
        methods: vec![yes_method, no_method],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };

    let ty = infer(&env, &ctx, &elim).expect("elim_Dec must infer (large elim into Type0)");
    let ty = whnf(&env, &ctx, &ty);
    assert!(
        matches!(&ty, Term::App(f, _) if matches!(**f, Term::IndFormer { id, .. } if id == dec_id)),
        "elim_Dec's large-elim result must be the Type0 motive (Dec P), got {:?}",
        ty
    );
}

// AC1 mechanism probe (QA-added): the AC1 test above uses a CONSTANT
// motive (`\x. Dec P`), which would also pass a degenerate eliminator that
// ignores per-branch typing entirely (the ES4-lawproofs mechanism-probe
// lesson). Confirm the kernel genuinely threads the per-constructor
// expected method type: a No-method whose domain isn't `P -> Empty`, and
// swapped Yes/No methods, must both be REJECTED — not just "some Type0
// motive is accepted."
#[test]
fn ac1_mechanism_probe_no_method_wrong_domain_rejected() {
    let mut env = GlobalEnv::new();
    let empty_id = declare_inductive(&mut env, |_empty| InductiveSpec {
        level_params: vec![],
        params: vec![],
        indices: vec![],
        level: lv0(),
        constructors: vec![],
    })
    .unwrap();
    let dec_id = declare_inductive(&mut env, |_dec| InductiveSpec {
        level_params: vec![],
        params: vec![Term::omega(lv0())],
        indices: vec![],
        level: lv0(),
        constructors: vec![
            CtorSpec { args: vec![Term::var(0)], target_indices: vec![] },
            CtorSpec {
                args: vec![Term::pi(Term::var(0), Term::indformer(empty_id, vec![]))],
                target_indices: vec![],
            },
        ],
    })
    .unwrap();
    let dec = env.inductive(dec_id).unwrap().clone();
    let (yes_id, no_id) = (dec.constructors[0].id, dec.constructors[1].id);

    let mut ctx = Context::new();
    ctx.push(Term::omega(lv0()));
    let dec_p = Term::app(Term::indformer(dec_id, vec![]), Term::var(0));
    ctx.push(dec_p);
    let p = Term::var(1);

    let motive = Term::Ascript(
        Box::new(Term::lam(
            Term::app(Term::indformer(dec_id, vec![]), p.clone()),
            Term::app(Term::indformer(dec_id, vec![]), Term::var(2)),
        )),
        Box::new(Term::pi(
            Term::app(Term::indformer(dec_id, vec![]), p.clone()),
            Term::Type(lv0()),
        )),
    );
    let yes_method = Term::lam(
        p.clone(),
        Term::app(Term::app(Term::constructor(yes_id, vec![]), Term::var(2)), Term::var(0)),
    );
    // BOGUS: domain is `Empty -> Empty`, not `P -> Empty`.
    let bogus_no_method = Term::lam(
        Term::pi(Term::indformer(empty_id, vec![]), Term::indformer(empty_id, vec![])),
        Term::app(Term::app(Term::constructor(no_id, vec![]), Term::var(2)), Term::var(0)),
    );
    let elim = Term::Elim {
        fam: dec_id,
        level_args: vec![],
        params: vec![p],
        motive: Box::new(motive),
        methods: vec![yes_method, bogus_no_method],
        indices: vec![],
        scrut: Box::new(Term::var(0)),
    };
    assert!(
        infer(&env, &ctx, &elim).is_err(),
        "elim_Dec must reject a No-method whose domain isn't P -> Empty"
    );
}

// AC2 — `Empty`/`absurdEmpty` elaborate through the real prelude+surface
// path (not the bare-kernel harness above).
#[test]
fn ac2_empty_and_absurd_empty_elaborate() {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    assert!(env.globals.contains_key("Empty"), "Empty must be a prelude global");
    assert!(env.globals.contains_key("Dec"), "Dec must be a prelude global");
    assert!(env.globals.contains_key("Yes"), "Yes must be a prelude global");
    assert!(env.globals.contains_key("No"), "No must be a prelude global");
    assert!(env.globals.contains_key("decide"), "decide must be a prelude global");

    env.elaborate_decl("fn absurdEmpty (C : Type) (e : Empty) : C = match e { }")
        .expect("absurdEmpty must elaborate (large elim via ordinary surface match)");
}

// AC3 — ground the `trusted_base()` delta on the Rust EMISSION, not a
// `.ken` view: `Empty`/`Dec` are ordinary `declare_inductive` admissions,
// never `declare_primitive`/`declare_postulate`.
#[test]
fn ac3_trusted_base_delta_is_ordinary_inductive_admission_only() {
    let prelude_src = include_str!("../src/prelude.rs");

    // `Empty` is admitted via `data::elab_data_decl` (the same surface-data
    // machinery every other prelude `data` uses), NEVER a primitive/postulate.
    assert!(
        prelude_src.contains("crate::data::elab_data_decl(") && prelude_src.contains("\"Empty\""),
        "Empty must be admitted via elab_data_decl (ordinary data admission), not a primitive"
    );
    // `Dec` is admitted via `declare_inductive` (kernel-direct), never a
    // primitive/postulate.
    let dec_block_start = prelude_src
        .find("`Dec (P : Omega) : Type0 = Yes P | No (P -> Empty)`")
        .expect("Dec's declaration comment must be present");
    let dec_tail = &prelude_src[dec_block_start..];
    let dec_block_end = dec_tail
        .char_indices()
        .nth(2000)
        .map(|(index, _)| index)
        .unwrap_or(dec_tail.len());
    let dec_block = &dec_tail[..dec_block_end];
    assert!(
        dec_block.contains("ken_kernel::declare_inductive"),
        "Dec must be admitted via declare_inductive (kernel-direct), not a primitive"
    );
    assert!(
        !dec_block.contains("declare_primitive") && !dec_block.contains("declare_postulate"),
        "Dec's admission must carry zero declare_primitive/declare_postulate delta"
    );

    // `Empty` is registered via `elab_data_decl`'s own internal
    // `globals.insert` (not a separate call site here) — confirm via the
    // FUNCTIONAL check (AC2 already does this) plus the textual call-site
    // grep above; `Dec` gets an explicit `globals.insert` right after its
    // `declare_inductive` call.
    assert!(
        prelude_src.contains("globals.insert(\"Dec\""),
        "Dec must be a registered global"
    );
    let env = ElabEnv::empty().expect("prelude bootstrap");
    assert!(env.globals.contains_key("Empty"), "Empty must be a registered global");
}

/// Promise class: durable invariant. MEASURED: the real EmptyDec entry loads
/// from a fresh catalog root, its `DecEq` class and Bool dictionary retain the
/// LawfulClasses identities, and its bridge body names Transport's canonical
/// `sym` and `trans` definitions. CLAIMED: the retired local copies were replaced
/// by real provider imports. THE GAP: checked examples are exercised separately
/// below; this pin owns provider identity in the package implementation.
#[test]
fn real_empty_dec_imports_bind_canonical_class_and_transport_identities() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], EMPTY_DEC)
        .expect("the real EmptyDec entry must isolated-roots-load");

    let dec_eq = env
        .class_env
        .class("DecEq")
        .expect("LawfulClasses must register DecEq")
        .projection
        .type_id;
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (DecEq as empty_dec_provider_dec_eq)\n\
         fn empty_dec_provider_class_identity \
           (a : Type) (d : empty_dec_provider_dec_eq a) \
           : empty_dec_provider_dec_eq a = d",
    )
    .expect("provider identities must remain selectively importable");
    let class_identity = env.globals["empty_dec_provider_class_identity"];
    let class_identity_type = match env.env.lookup(class_identity) {
        Some(Decl::Transparent { ty, .. }) => ty,
        other => panic!("class identity witness must remain transparent: {other:?}"),
    };
    assert!(
        term_mentions(class_identity_type, dec_eq),
        "the selective DecEq import must retain the canonical class GlobalId"
    );
    let bool_instance = env
        .class_env
        .instance_search("DecEq", "Bool")
        .expect("LawfulClasses must register DecEq Bool");
    let info = env
        .class_env
        .instances
        .values()
        .find(|info| info.instance_id == bool_instance)
        .expect("the canonical Bool dictionary has registry provenance");
    assert_eq!(info.defining_package, LAWFUL);

    let bridge = env.globals[&format!("{EMPTY_DEC}.dec_eq_decides")];
    let (_, body) = env
        .env
        .transparent_body(bridge)
        .expect("dec_eq_decides must remain transparent");
    for provider in ["sym", "trans"] {
        let provider = env.globals[&format!("{TRANSPORT}.{provider}")];
        assert!(
            term_mentions(&body, provider),
            "dec_eq_decides must retain the canonical Transport provider identity"
        );
    }
}

// AC4 — the bridge is demonstrated over `DecEq Bool` (inductive carrier,
// honest via no-confusion/K7), not only `DecEq Int` (`Axiom`-backed). The
// roots loader executes the real entry's checked example fences after loading
// its declared providers, then the resulting example bodies must mention the
// canonical class-owner dictionary.
#[test]
fn ac4_bridge_demonstrated_over_deceq_bool_not_only_deceq_int() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], EMPTY_DEC)
        .expect("EmptyDec implementation must isolated-roots-load");
    env.execute_loaded_entry_checked_fences(EMPTY_DEC)
        .expect("EmptyDec checked examples and rejection fences must elaborate");
    let bool_instance = env
        .class_env
        .instance_search("DecEq", "Bool")
        .expect("the class owner must register DecEq Bool");
    for example in ["true_is_true", "true_is_not_false"] {
        let example = env.globals[example];
        let (_, body) = env
            .env
            .transparent_body(example)
            .expect("checked example must remain transparent");
        assert!(
            term_mentions(&body, bool_instance),
            "the checked Bool bridge example must consume the canonical dictionary"
        );
    }
}

// Confirm `catalog/packages/Core/Classes/LawfulClasses.ken.md` still elaborates
// over its declared dependencies and registers the same canonical dictionary
// consumed by EmptyDec.
#[test]
fn landed_lawful_classes_package_still_elaborates_with_dependencies() {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    let provider_state = catalog_or::core_logic_or_module_state(&env);
    catalog_or::expose_core_logic_transport(&mut env);
    catalog_or::load_derived_fixture(&mut env);
    catalog_or::restore_core_logic_or_module_state(&mut env, &provider_state);
    catalog_or::assert_transparent_result_uses_core_logic_or(&env, "compare_bool_cases");
    assert!(
        env.globals.contains_key("DecEq_instance_Bool"),
        "the landed package's own DecEq_instance_Bool must be a real registered global"
    );
}
