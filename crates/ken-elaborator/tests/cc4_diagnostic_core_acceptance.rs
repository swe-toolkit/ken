//! CC4 (`Capability.Diagnostics.Core`) ordered shared-environment acceptance.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{EvalStore, EvalVal, ListCharIds, eval};
use ken_kernel::{Decl, GlobalId, Term};

const DIAGNOSTIC_KEN_MD: &str =
    include_str!("../../../catalog/packages/Capability/Diagnostics/Core.ken.md");
const NUMERIC_KEN_MD: &str =
    include_str!("../../../catalog/packages/Capability/Parsing/Numeric.ken.md");

fn dependency_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let (lawful_owned, _) =
        catalog_or::load_derived_importing_fixture_many(&mut env, &["list_append", "length"]);
    catalog_or::assert_derived_fixture_retains_lawfulclasses(&mut env, &lawful_owned);
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Numeric.Nat.Arithmetic")
        .expect("Data.Numeric.Nat.Arithmetic must load as a qualified module");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Numeric.Nat.Order")
        .expect("Data.Numeric.Nat.Order must load as a qualified module");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("Core.Classes.LawfulClasses must load as a qualified module");
    let lawful_prefix = "Core.Classes.LawfulClasses.";
    let lawful_aliases: Vec<_> = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            name.strip_prefix(lawful_prefix)
                .map(|suffix| (suffix.to_owned(), *id))
        })
        .collect();
    env.globals.extend(lawful_aliases);
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Diagnostics.Core")
        .expect("Capability.Diagnostics.Core must roots-load fourth");
    catalog_or::expose_module(&mut env, "Capability.Diagnostics.Core");
    env
}

fn load_cursor_module(env: &mut ElabEnv) -> BTreeSet<GlobalId> {
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Parsing.Cursor")
        .expect("Capability.Parsing.Cursor must roots-load")
        .into_iter()
        .collect();
    catalog_or::expose_module(env, "Capability.Parsing.Cursor");
    owned
}

fn load_decoder_module(env: &mut ElabEnv) -> BTreeSet<GlobalId> {
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Parsing.Decoder")
        .expect("Capability.Parsing.Decoder must roots-load")
        .into_iter()
        .collect();
    catalog_or::expose_module(env, "Capability.Parsing.Decoder");
    owned
}

fn load_parsing_module(env: &mut ElabEnv) -> BTreeSet<GlobalId> {
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Parsing.Parsing")
        .expect("Capability.Parsing.Parsing must roots-load")
        .into_iter()
        .collect();
    catalog_or::expose_module(env, "Capability.Parsing.Parsing");
    owned
}

#[test]
fn derived_fixture_retains_lawfulclasses_for_cc4_dependency_closure() {
    let _ = dependency_env();
}

fn full_env() -> ElabEnv {
    let mut env = dependency_env();
    load_cursor_module(&mut env);
    load_decoder_module(&mut env);
    load_parsing_module(&mut env);
    env.elaborate_ken_md_file(NUMERIC_KEN_MD)
        .expect("Capability.Parsing.Numeric must elaborate eighth");
    env
}

fn assert_transparent_globals(env: &ElabEnv, names: &[&str]) {
    for name in names {
        let id = *env
            .globals
            .get(*name)
            .unwrap_or_else(|| panic!("expected checked global `{name}`"));
        assert!(
            env.env.transparent_body(id).is_some(),
            "`{name}` must be a real transparent, kernel-checked term"
        );
    }
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

fn module_ids(env: &ElabEnv, module: &str) -> BTreeSet<GlobalId> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect()
}

fn owned_names(env: &ElabEnv, module: &str, owned: &BTreeSet<GlobalId>) -> BTreeSet<String> {
    let prefix = format!("{module}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            (owned.contains(id) && name.starts_with(&prefix))
                .then(|| name[prefix.len()..].to_owned())
        })
        .collect()
}

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn leading_result(mut term: &Term) -> &Term {
    while let Term::Pi(_, result) = term {
        term = result;
    }
    term
}

fn is_global(term: &Term, target: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } | Term::IndFormer { id, .. } if *id == target)
}

fn lit_to_eval(value: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match value {
        NumericLitVal::Int(n) => EvalVal::from(n.clone()),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
    }
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        store
            .num_values
            .insert(*id, lit_to_eval(value, mkdecimalpair_id));
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn eval_global(env: &ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let id = env.globals[name];
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("`{name}` must be transparent, got {other:?}"),
    }
}

fn ctor_args<'a>(env: &ElabEnv, value: &'a EvalVal, name: &str) -> &'a [EvalVal] {
    let expected = env.globals[name];
    match value {
        EvalVal::Ctor { id, args, .. } if *id == expected => args.as_ref().as_slice(),
        other => panic!("expected `{name}`, got {other:?}"),
    }
}

fn nat_count(env: &ElabEnv, value: &EvalVal) -> u64 {
    match value {
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_count(env, &args[0])
        }
        other => panic!("expected Nat, got {other:?}"),
    }
}

#[test]
fn ordered_dependency_closure_elaborates_all_cc4_clients() {
    let env = full_env();
    assert_transparent_globals(
        &env,
        &[
            "byte_range_start",
            "byte_range_end",
            "origin_source_id",
            "origin_argument_index",
            "origin_range_start",
            "origin_range_end",
            "environment_origin",
            "config_key_origin",
            "diagnostic_origin",
            "diagnostic_code",
            "ValidByteRange",
            "ValidConfigKeyPath",
            "ValidOrigin",
            "ValidDiagnostic",
            "arg_location_origin",
            "arg_location_origin_index_faithful",
            "arg_location_origin_start_faithful",
            "arg_location_origin_end_faithful",
            "span_to_byte_range",
            "span_origin",
            "span_to_byte_range_faithful",
            "span_origin_source_faithful",
            "numeric_error_code",
            "numeric_diagnostic",
            "numeric_argument_origin",
            "numeric_argument_origin_index_faithful",
            "numeric_argument_origin_start_faithful",
            "numeric_argument_origin_end_faithful",
        ],
    );

    for name in [
        "SourceId",
        "ByteRange",
        "Origin",
        "DiagnosticCode",
        "Diagnostic",
    ] {
        assert!(
            env.globals.contains_key(name),
            "expected checked data `{name}`"
        );
    }
    assert!(
        !env.globals.contains_key("NumericError"),
        "Capability.Parsing.Numeric must not retain its pre-CC4 carrier"
    );
}

/// Durable invariant: the roots-loaded Diagnostics package references the
/// canonical lawful owner directly, without minting a local relation or adding
/// trust.
#[test]
fn diagnostics_reuses_the_canonical_lawful_classes_relation() {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("canonical Nat relation provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Diagnostics.Core")
        .expect("Capability.Diagnostics.Core must roots-load with its import closure");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Diagnostics reuse must add zero trust");

    let provider = env.globals["Core.Classes.LawfulClasses.leq_nat"];
    assert!(env.env.transparent_body(provider).is_some());
    assert!(
        !env.globals
            .contains_key("Capability.Diagnostics.Core.diagnostic_nat_leq"),
        "Diagnostics must not mint a local Nat relation"
    );
    assert!(
        !env.globals
            .keys()
            .any(|name| name.starts_with("Data.Numeric.Nat.Order.")),
        "Diagnostics must import the canonical owner without loading the Order facade"
    );

    let valid_range = env.globals["Capability.Diagnostics.Core.ValidByteRange"];
    let body = match env.env.lookup(valid_range) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("ValidByteRange must be transparent, got {other:?}"),
    };
    assert!(
        term_mentions(body, provider),
        "ValidByteRange must retain the canonical provider GlobalId"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: Parsing owns no `SourceId` declaration while its checked closure
/// retains Diagnostics.Core's canonical identity, and Decoder-owned declarations
/// retain no Diagnostics.Core identity. CLAIMED: SourceId is defined once below
/// Parsing, while the location-generic Decoder remains diagnostics-independent.
/// THE GAP: identity closure does not prove injected locations are faithful;
/// the non-degenerate evaluation test below owns that behavioral obligation.
#[test]
fn cc4_clients_use_canonical_diagnostics_without_duplicate_carriers() {
    let root = catalog_or::catalog_root();
    let mut parsing_env = ElabEnv::new().expect("base environment");
    let parsing_owned = parsing_env
        .elaborate_module_from_roots(std::slice::from_ref(&root), "Capability.Parsing.Parsing")
        .expect("Parsing must roots-load")
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert!(
        !owned_names(&parsing_env, "Capability.Parsing.Parsing", &parsing_owned,)
            .contains("SourceId"),
        "Parsing must not own a duplicate SourceId"
    );
    let canonical_source_id = parsing_env.globals["Capability.Diagnostics.Core.SourceId"];
    assert!(
        catalog_or::owned_references(&parsing_env, &parsing_owned).contains(&canonical_source_id),
        "Parsing must retain the canonical Diagnostics.Core.SourceId GlobalId"
    );

    let mut decoder_env = ElabEnv::new().expect("base environment");
    let decoder_owned = decoder_env
        .elaborate_module_from_roots(std::slice::from_ref(&root), "Capability.Parsing.Decoder")
        .expect("Decoder must roots-load")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let diagnostic_ids = module_ids(&decoder_env, "Capability.Diagnostics.Core");
    let retained = catalog_or::owned_references(&decoder_env, &decoder_owned)
        .intersection(&diagnostic_ids)
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(
        retained.is_empty(),
        "Decoder-owned checked declarations must remain independent of Diagnostics.Core: {retained:?}"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: the real CC4 load leaves the kernel's trusted-base population
/// unchanged. CLAIMED: every CC4 declaration remains kernel-checked rather than
/// admitted. THE GAP: trust equality does not establish carrier behavior, which
/// is exercised independently by the shape and non-degenerate injection tests.
#[test]
fn checked_cc4_chain_has_zero_trusted_base_delta() {
    let mut diagnostics = ElabEnv::empty().expect("prelude bootstrap");
    diagnostics
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("Diagnostics provider must roots-load");
    let before: BTreeSet<_> = diagnostics.env.trusted_base().into_iter().collect();
    diagnostics
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Diagnostics.Core")
        .expect("Diagnostics.Core must roots-load");
    let after: BTreeSet<_> = diagnostics.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Diagnostics.Core must add zero trust");

    let mut cursor = dependency_env();
    let before: BTreeSet<_> = cursor.env.trusted_base().into_iter().collect();
    load_cursor_module(&mut cursor);
    let after: BTreeSet<_> = cursor.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Parsing.Cursor must add zero trust");

    let mut parsing = dependency_env();
    load_cursor_module(&mut parsing);
    load_decoder_module(&mut parsing);
    // Parsing imports four audited byte-provider facts. Measure its own trust
    // delta after loading that provider, rather than attributing them to Parsing.
    parsing
        .elaborate_module_from_roots(
            &[catalog_or::catalog_root()],
            "Data.Binary.BytesPrimitiveContracts",
        )
        .expect("Parsing byte-contract provider must roots-load");
    let before: BTreeSet<_> = parsing.env.trusted_base().into_iter().collect();
    load_parsing_module(&mut parsing);
    let after: BTreeSet<_> = parsing.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Parsing.Parsing must add zero trust");

    let mut numeric = dependency_env();
    let before: BTreeSet<_> = numeric.env.trusted_base().into_iter().collect();
    numeric
        .elaborate_ken_md_file(NUMERIC_KEN_MD)
        .expect("Parsing.Numeric must elaborate");
    let after: BTreeSet<_> = numeric.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "Parsing.Numeric must add zero trust");
}

#[test]
fn exact_non_degenerate_injections_preserve_every_location_field() {
    fn owned_constructor(
        env: &ElabEnv,
        owned: &[GlobalId],
        module: &str,
        family: &str,
        name: &str,
    ) -> GlobalId {
        let family_id = catalog_or::provider_owned_id(env, owned, module, family)
            .unwrap_or_else(|error| panic!("{module} must own {family}: {error}"));
        let constructors = env
            .env
            .inductive(family_id)
            .expect("an owned constructor parent must be an inductive")
            .constructors
            .iter()
            .map(|constructor| constructor.id)
            .collect::<Vec<_>>();
        catalog_or::provider_owned_id(env, &constructors, module, name)
            .unwrap_or_else(|error| panic!("{module}.{name} must belong to {family}: {error}"))
    }
    fn owned_ctor_args(value: &EvalVal, expected: GlobalId) -> &[EvalVal] {
        match value {
            EvalVal::Ctor { id, args, .. } if *id == expected => args.as_ref().as_slice(),
            other => panic!("expected checked constructor {expected:?}, got {other:?}"),
        }
    }
    fn checked_owner_example(env: &ElabEnv, owned: &[GlobalId], module: &str, name: &str) -> Term {
        let id = *env
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("{module} checked example {name} must elaborate"));
        assert!(!owned.contains(&id), "{name} must not be provider-owned");
        assert!(
            !env.globals.contains_key(&format!("{module}.{name}")),
            "{name} must not be a qualified provider declaration"
        );
        let (_, body) = env
            .env
            .transparent_body(id)
            .expect("owner example must be a checked transparent definition");
        body
    }
    let root = catalog_or::catalog_root();
    let dc = "Capability.Diagnostics.Core";
    let cursor = "Capability.Parsing.Cursor";
    let mut dc_owner = ElabEnv::new().expect("DC owner base environment");
    let dc_owned = dc_owner
        .elaborate_module_from_roots(std::slice::from_ref(&root), dc)
        .expect("Diagnostics.Core must roots-load");
    let dc_trust_before: BTreeSet<_> = dc_owner.env.trusted_base().into_iter().collect();
    dc_owner
        .execute_loaded_entry_checked_fences(dc)
        .expect("Diagnostics.Core's private examples must check in owner scope");
    assert_eq!(
        dc_owner
            .env
            .trusted_base()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        dc_trust_before,
        "DC examples must add no trust"
    );
    let dc_source_id = owned_constructor(&dc_owner, &dc_owned, dc, "SourceId", "MkSourceId");
    let dc_env_origin =
        catalog_or::provider_owned_id(&dc_owner, &dc_owned, dc, "environment_origin")
            .expect("DC must own its private environment-origin helper");
    let dc_config_origin =
        catalog_or::provider_owned_id(&dc_owner, &dc_owned, dc, "config_key_origin")
            .expect("DC must own its private config-origin helper");
    let dc_source_body = checked_owner_example(
        &dc_owner,
        &dc_owned,
        dc,
        "diagnostics_example_source_id_four",
    );
    let dc_environment_body = checked_owner_example(
        &dc_owner,
        &dc_owned,
        dc,
        "diagnostics_example_environment_origin_path",
    );
    let dc_config_body = checked_owner_example(
        &dc_owner,
        &dc_owned,
        dc,
        "diagnostics_example_config_key_origin_path",
    );
    assert!(term_mentions(&dc_source_body, dc_source_id));
    assert!(term_mentions(&dc_environment_body, dc_env_origin));
    assert!(term_mentions(&dc_config_body, dc_config_origin));

    let mut cursor_owner = ElabEnv::new().expect("Cursor owner base environment");
    let cursor_owned = cursor_owner
        .elaborate_module_from_roots(std::slice::from_ref(&root), cursor)
        .expect("Cursor must roots-load with its public DC dependency");
    let cursor_trust_before: BTreeSet<_> = cursor_owner.env.trusted_base().into_iter().collect();
    cursor_owner
        .execute_loaded_entry_checked_fences(cursor)
        .expect("Cursor's private example must check in owner scope");
    assert_eq!(
        cursor_owner
            .env
            .trusted_base()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        cursor_trust_before,
        "Cursor examples must add no trust"
    );
    let cursor_origin =
        catalog_or::provider_owned_id(&cursor_owner, &cursor_owned, cursor, "arg_location_origin")
            .expect("Cursor must own its private argument-origin bridge");
    let cursor_dc_owned = cursor_owner
        .elaborate_module_from_roots(std::slice::from_ref(&root), dc)
        .expect("Cursor's public DC dependency must retain its checked owner");
    let public_source_id =
        catalog_or::provider_owned_id(&cursor_owner, &cursor_dc_owned, dc, "source_id_from_nat")
            .expect("DC must own its public source identifier constructor function");
    let cursor_source_body = checked_owner_example(
        &cursor_owner,
        &cursor_owned,
        cursor,
        "cursor_example_source_origin_span",
    );
    let cursor_argument_body = checked_owner_example(
        &cursor_owner,
        &cursor_owned,
        cursor,
        "cursor_example_argument_origin_range",
    );
    let cursor_private_dc_source = owned_constructor(
        &cursor_owner,
        &cursor_dc_owned,
        dc,
        "SourceId",
        "MkSourceId",
    );
    assert!(term_mentions(&cursor_source_body, public_source_id));
    assert!(!term_mentions(
        &cursor_source_body,
        cursor_private_dc_source
    ));
    assert!(term_mentions(&cursor_argument_body, cursor_origin));
    assert!(!term_mentions(
        &cursor_argument_body,
        cursor_private_dc_source
    ));

    for (module, examples) in [
        (
            dc,
            &[
                "diagnostics_example_source_id_four",
                "diagnostics_example_environment_origin_path",
                "diagnostics_example_config_key_origin_path",
            ][..],
        ),
        (
            cursor,
            &[
                "cursor_example_source_origin_span",
                "cursor_example_argument_origin_range",
            ][..],
        ),
    ] {
        let mut client = ElabEnv::new().expect("example privacy client");
        client
            .elaborate_module_from_roots(std::slice::from_ref(&root), module)
            .expect("example privacy provider must roots-load");
        for name in examples {
            let error = client
                .elaborate_file(&format!("import {module} ({name})"))
                .expect_err("owner examples must not be public imports");
            assert!(
                matches!(error, ElabError::UnboundName { .. }),
                "{module}.{name} must refuse selective import, got {error:?}"
            );
        }
    }

    let mut env = full_env();
    env.elaborate_file(
        r#"import Capability.Diagnostics.Core
          (SourceId, Origin, SourceOrigin, ArgumentOrigin, EnvironmentOrigin,
           ConfigKeyOrigin, MkByteRange, MkDiagnostic, MkDiagnosticCode,
           ValidByteRange, ValidOrigin, source_id_from_nat)
        import Capability.Parsing.Cursor (ArgLocation, MkArgLocation)"#,
    )
    .expect("CC4's public DC and Cursor prerequisites must import");
    env.elaborate_file(
        r#"
        const cc4_source_origin : Origin =
          span_origin
            (source_id_from_nat (Suc (Suc (Suc (Suc Zero)))))
            (MkSpan (Suc (Suc Zero)) (Suc (Suc (Suc (Suc (Suc Zero))))))

        const cc4_argument_origin : Origin =
          ArgumentOrigin
            (Suc (Suc Zero))
            (MkByteRange
              (Suc (Suc (Suc Zero)))
              (Suc (Suc (Suc Zero))))

        theorem cc4_valid_range :
            ValidByteRange
              (MkByteRange
                (Suc (Suc Zero))
                (Suc (Suc (Suc (Suc (Suc Zero)))))) =
          Proved

        const cc4_environment_name : String = "PATH"
        const cc4_environment_origin : Origin = EnvironmentOrigin cc4_environment_name
        const cc4_config_origin : Origin = ConfigKeyOrigin (Cons String "PATH" (Nil String))

        theorem cc4_valid_environment :
            ValidOrigin (EnvironmentOrigin cc4_environment_name) =
          Proved
        "#,
    )
    .expect("non-degenerate CC4 probes must elaborate");

    let invalid = env.elaborate_decl(
        "theorem cc4_invalid_range : ValidByteRange (MkByteRange (Suc Zero) Zero) = Proved",
    );
    assert!(
        matches!(invalid, Err(ElabError::KernelRejected { .. })),
        "start > end must fail specifically at kernel checking, got {invalid:?}"
    );
    let empty_config = env.elaborate_decl(
        "theorem cc4_empty_config : ValidOrigin (ConfigKeyOrigin (Nil String)) = Proved",
    );
    assert!(
        matches!(empty_config, Err(ElabError::KernelRejected { .. })),
        "an empty config key path must fail at kernel checking, got {empty_config:?}"
    );

    let client_dc_owned = env
        .elaborate_module_from_roots(std::slice::from_ref(&root), dc)
        .expect("DC owner identities must remain available to the CC4 host");
    let client_source_id = owned_constructor(&env, &client_dc_owned, dc, "SourceId", "MkSourceId");
    let source_origin_id = owned_constructor(&env, &client_dc_owned, dc, "Origin", "SourceOrigin");
    let argument_origin_id =
        owned_constructor(&env, &client_dc_owned, dc, "Origin", "ArgumentOrigin");
    let environment_origin_id =
        owned_constructor(&env, &client_dc_owned, dc, "Origin", "EnvironmentOrigin");
    let config_origin_id =
        owned_constructor(&env, &client_dc_owned, dc, "Origin", "ConfigKeyOrigin");
    let byte_range_id = owned_constructor(&env, &client_dc_owned, dc, "ByteRange", "MkByteRange");
    let diagnostic_id = owned_constructor(&env, &client_dc_owned, dc, "Diagnostic", "MkDiagnostic");
    let diagnostic_code_id = owned_constructor(
        &env,
        &client_dc_owned,
        dc,
        "DiagnosticCode",
        "MkDiagnosticCode",
    );
    let mut store = make_store(&env);

    let source = eval_global(&env, &mut store, "cc4_source_origin");
    let source_fields = owned_ctor_args(&source, source_origin_id);
    let source_id = owned_ctor_args(&source_fields[0], client_source_id);
    assert_eq!(nat_count(&env, &source_id[0]), 4);
    let source_range = owned_ctor_args(&source_fields[1], byte_range_id);
    assert_eq!(nat_count(&env, &source_range[0]), 2);
    assert_eq!(nat_count(&env, &source_range[1]), 5);

    let argument = eval_global(&env, &mut store, "cc4_argument_origin");
    let argument_fields = owned_ctor_args(&argument, argument_origin_id);
    assert_eq!(nat_count(&env, &argument_fields[0]), 2);
    let argument_range = owned_ctor_args(&argument_fields[1], byte_range_id);
    assert_eq!(nat_count(&env, &argument_range[0]), 3);
    assert_eq!(nat_count(&env, &argument_range[1]), 3);

    let numeric = eval_global(&env, &mut store, "bad_digit_result");
    let diagnostic = ctor_args(&env, &numeric, "Err").last().unwrap();
    let diagnostic_fields = owned_ctor_args(diagnostic, diagnostic_id);
    let numeric_origin = owned_ctor_args(&diagnostic_fields[0], argument_origin_id);
    assert_eq!(nat_count(&env, &numeric_origin[0]), 2);
    let numeric_range = owned_ctor_args(&numeric_origin[1], byte_range_id);
    assert_eq!(nat_count(&env, &numeric_range[0]), 2);
    assert_eq!(nat_count(&env, &numeric_range[1]), 2);
    let code = owned_ctor_args(&diagnostic_fields[1], diagnostic_code_id);
    assert_eq!(
        code.last(),
        Some(&EvalVal::Str("text.numeric.invalid-digit".into()))
    );

    let environment = eval_global(&env, &mut store, "cc4_environment_origin");
    assert_eq!(
        owned_ctor_args(&environment, environment_origin_id),
        &[EvalVal::Str("PATH".into())]
    );
    let config = eval_global(&env, &mut store, "cc4_config_origin");
    let config_fields = owned_ctor_args(&config, config_origin_id);
    let config_head = ctor_args(&env, &config_fields[0], "Cons");
    assert_eq!(config_head[1], EvalVal::Str("PATH".into()));
    ctor_args(&env, &config_head[2], "Nil");

    let mut dc_store = make_store(&dc_owner);
    let dc_source = eval(&[], &dc_source_body, &dc_owner.env, &mut dc_store);
    let dc_id_fields = owned_ctor_args(&dc_source, dc_source_id);
    assert_eq!(nat_count(&dc_owner, &dc_id_fields[0]), 4);
    let dc_environment = eval(&[], &dc_environment_body, &dc_owner.env, &mut dc_store);
    let dc_environment_id =
        owned_constructor(&dc_owner, &dc_owned, dc, "Origin", "EnvironmentOrigin");
    assert_eq!(
        owned_ctor_args(&dc_environment, dc_environment_id),
        &[EvalVal::Str("PATH".into())]
    );
    let dc_config = eval(&[], &dc_config_body, &dc_owner.env, &mut dc_store);
    let dc_config_id = owned_constructor(&dc_owner, &dc_owned, dc, "Origin", "ConfigKeyOrigin");
    let dc_config_fields = owned_ctor_args(&dc_config, dc_config_id);
    let dc_config_head = ctor_args(&dc_owner, &dc_config_fields[0], "Cons");
    assert_eq!(dc_config_head[1], EvalVal::Str("PATH".into()));
    ctor_args(&dc_owner, &dc_config_head[2], "Nil");

    let mut cursor_store = make_store(&cursor_owner);
    let cursor_source = eval(
        &[],
        &cursor_source_body,
        &cursor_owner.env,
        &mut cursor_store,
    );
    let cursor_source_origin_id = owned_constructor(
        &cursor_owner,
        &cursor_dc_owned,
        dc,
        "Origin",
        "SourceOrigin",
    );
    let cursor_source_fields = owned_ctor_args(&cursor_source, cursor_source_origin_id);
    let cursor_source_id = owned_constructor(
        &cursor_owner,
        &cursor_dc_owned,
        dc,
        "SourceId",
        "MkSourceId",
    );
    let cursor_source_value = owned_ctor_args(&cursor_source_fields[0], cursor_source_id);
    assert_eq!(nat_count(&cursor_owner, &cursor_source_value[0]), 4);
    let cursor_range_id = owned_constructor(
        &cursor_owner,
        &cursor_dc_owned,
        dc,
        "ByteRange",
        "MkByteRange",
    );
    let cursor_range = owned_ctor_args(&cursor_source_fields[1], cursor_range_id);
    assert_eq!(nat_count(&cursor_owner, &cursor_range[0]), 2);
    assert_eq!(nat_count(&cursor_owner, &cursor_range[1]), 5);

    let cursor_argument = eval(
        &[],
        &cursor_argument_body,
        &cursor_owner.env,
        &mut cursor_store,
    );
    let cursor_argument_id = owned_constructor(
        &cursor_owner,
        &cursor_dc_owned,
        dc,
        "Origin",
        "ArgumentOrigin",
    );
    let cursor_argument_fields = owned_ctor_args(&cursor_argument, cursor_argument_id);
    assert_eq!(nat_count(&cursor_owner, &cursor_argument_fields[0]), 2);
    let cursor_argument_range = owned_ctor_args(&cursor_argument_fields[1], cursor_range_id);
    assert_eq!(nat_count(&cursor_owner, &cursor_argument_range[0]), 3);
    assert_eq!(nat_count(&cursor_owner, &cursor_argument_range[1]), 3);
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every publishable Diagnostics.Core declaration is queried through
/// the roots loader, the successful surface equals the curated carrier API, and
/// a selective client type-checks every carrier constructor at its intended
/// argument/result types. CLAIMED: Diagnostics.Core exposes the structured
/// diagnostic carrier and no presentation API. THE GAP: private declarations
/// are not a public surface; checked dependency closure is covered separately by
/// the canonical-lawful-provider test.
#[test]
fn diagnostic_core_loader_surface_and_carrier_shapes_are_exact() {
    let expected = names(&[
        "ArgumentOrigin",
        "ByteRange",
        "ConfigKeyOrigin",
        "Diagnostic",
        "DiagnosticCode",
        "EnvironmentOrigin",
        "MkByteRange",
        "MkDiagnostic",
        "MkDiagnosticCode",
        "Origin",
        "SourceId",
        "SourceOrigin",
        "byte_range_end",
        "byte_range_start",
        "diagnostic_code",
        "diagnostic_origin",
        "origin_argument_index",
        "origin_range_end",
        "origin_range_start",
        "origin_source_id",
        "ValidByteRange",
        "ValidConfigKeyPath",
        "ValidDiagnostic",
        "ValidOrigin",
        "source_id_from_nat",
    ]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            DIAGNOSTIC_KEN_MD,
            "Capability.Diagnostics.Core",
            "cc4_diagnostics_core",
        ),
        expected,
    );

    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Diagnostics.Core")
        .expect("Diagnostics.Core must roots-load")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let formatting_ids = module_ids(&env, "Capability.Formatting.Doc");
    let string = env.globals["String"];
    let presentation_declarations = owned
        .iter()
        .filter(|id| {
            let declaration = env
                .env
                .lookup(**id)
                .unwrap_or_else(|| panic!("owned global {id:?} must resolve"));
            !catalog_or::declaration_references(declaration).is_disjoint(&formatting_ids)
                || matches!(declaration, Decl::Transparent { ty, .. }
                    if is_global(leading_result(ty), string))
        })
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(
        presentation_declarations.is_empty(),
        "Diagnostics.Core must own no checked presentation dependency or String renderer: {presentation_declarations:?}"
    );

    env.elaborate_file(
        r#"
        import Capability.Diagnostics.Core
          (ArgumentOrigin,
            ByteRange,
            ConfigKeyOrigin,
            Diagnostic,
            DiagnosticCode,
            EnvironmentOrigin,
            MkDiagnostic,
            Origin,
            SourceId,
            SourceOrigin)

        fn cc4_source_origin_shape (source : SourceId) (range : ByteRange) : Origin =
          SourceOrigin source range

        fn cc4_argument_origin_shape (index : Nat) (range : ByteRange) : Origin =
          ArgumentOrigin index range

        fn cc4_environment_origin_shape (variable : String) : Origin =
          EnvironmentOrigin variable

        fn cc4_config_key_origin_shape (path : List String) : Origin =
          ConfigKeyOrigin path

        fn cc4_diagnostic_shape (origin : Origin) (code : DiagnosticCode) : Diagnostic =
          MkDiagnostic origin code
        "#,
    )
    .expect("selective client must consume every structured carrier shape");
}
