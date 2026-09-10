//! CC5 (`Capability.Formatting.Doc`) ordered shared-environment acceptance.

#[path = "support/catalog_or.rs"]
mod catalog_or;
#[path = "support/catalog_publication.rs"]
mod catalog_publication;

use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, NumericLitVal};
use ken_interp::eval::{EvalStore, EvalVal, ListCharIds, eval};
use ken_kernel::{Decl, GlobalId, Term};

const PRETTY_DOC_KEN_MD: &str =
    include_str!("../../../catalog/packages/Capability/Formatting/Doc.ken.md");

fn dependency_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("Core.Classes.LawfulClasses must load as a qualified module");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Numeric.Nat.Arithmetic")
        .expect("Data.Numeric.Nat.Arithmetic must load as a qualified module");
    catalog_or::load_derived_importing_fixture_many(&mut env, &["length", "list_append"]);
    env
}

fn full_env() -> ElabEnv {
    let mut env = dependency_env();
    env.elaborate_ken_md_file(PRETTY_DOC_KEN_MD)
        .expect("Capability.Formatting.Doc and every checked fence must elaborate third");
    catalog_or::assert_transparent_result_uses_core_logic_or(&env, "pretty_bool_cases");
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

fn declaration_terms(declaration: &Decl) -> Vec<&Term> {
    match declaration {
        Decl::Transparent { ty, body, .. } => vec![ty, body],
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => vec![ty],
        Decl::Inductive(inductive) => {
            let mut terms = Vec::new();
            terms.extend(inductive.params.iter());
            terms.extend(inductive.indices.iter());
            terms.push(&inductive.former_type);
            for constructor in &inductive.constructors {
                terms.extend(constructor.args.iter());
                terms.extend(constructor.target_indices.iter());
                terms.push(&constructor.type_);
            }
            terms
        }
    }
}

fn application_spine(term: &Term) -> (&Term, Vec<&Term>) {
    let mut head = term;
    let mut arguments = Vec::new();
    while let Term::App(function, argument) = head {
        arguments.push(argument.as_ref());
        head = function;
    }
    arguments.reverse();
    (head, arguments)
}

fn is_global(term: &Term, target: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } | Term::IndFormer { id, .. } if *id == target)
}

fn contains_equal_string(term: &Term, equal: GlobalId, string: GlobalId) -> bool {
    let (head, arguments) = application_spine(term);
    (is_global(head, equal) && arguments.first().is_some_and(|arg| is_global(arg, string)))
        || term
            .children()
            .into_iter()
            .any(|child| contains_equal_string(child, equal, string))
}

fn names(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
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

fn list_char_codepoints(env: &ElabEnv, value: &EvalVal) -> Vec<u32> {
    let nil_id = env.prelude_env.nil_id;
    let cons_id = env.prelude_env.cons_id;
    let mut out = Vec::new();
    let mut current = value.clone();
    loop {
        match &current {
            EvalVal::Ctor { id, .. } if *id == nil_id => return out,
            EvalVal::Ctor { id, args, .. } if *id == cons_id => {
                match &args[1] {
                    EvalVal::Int(n) => out.push(*n as u32),
                    other => panic!("Cons head must be an Int-typed Char, got {other:?}"),
                }
                current = args[2].clone();
            }
            other => panic!("expected a List Char constructor chain, got {other:?}"),
        }
    }
}

fn as_text(env: &ElabEnv, value: &EvalVal) -> String {
    list_char_codepoints(env, value)
        .into_iter()
        .map(|codepoint| char::from_u32(codepoint).expect("valid Char codepoint"))
        .collect()
}

fn add_render_probes(env: &mut ElabEnv) {
    env.elaborate_file(
        r#"
        const cc5_group_doc : Doc =
          Group
            (Concat
              (Text (string_to_list_char "ab"))
              (Nest
                (Suc (Suc Zero))
                (Concat Line (Text (string_to_list_char "cd")))))

        const cc5_alt_doc : Doc =
          Alt
            (Concat
              (Text (string_to_list_char "ab"))
              (Concat Line (Text (string_to_list_char "cd"))))
            (Concat
              (Text (string_to_list_char "ab"))
              (Nest
                (Suc (Suc Zero))
                (Concat Line (Text (string_to_list_char "cd")))))

        const cc5_group_below : List Char =
          render (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc

        const cc5_group_boundary : List Char =
          render (Suc (Suc (Suc (Suc (Suc Zero))))) cc5_group_doc

        const cc5_group_above : List Char =
          render (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))) cc5_group_doc

        const cc5_group_boundary_again : List Char =
          render (Suc (Suc (Suc (Suc (Suc Zero))))) cc5_group_doc

        const cc5_alt_below : List Char =
          render (Suc (Suc (Suc (Suc Zero)))) cc5_alt_doc

        const cc5_alt_boundary : List Char =
          render (Suc (Suc (Suc (Suc (Suc Zero))))) cc5_alt_doc

        const cc5_alt_above : List Char =
          render (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))) cc5_alt_doc

        "#,
    )
    .expect("boundary and determinism probes must elaborate");
}

fn add_law_probes(env: &mut ElabEnv) {
    env.elaborate_file(
        r#"
        theorem cc5_group_valid : DocContentInvariant cc5_group_doc =
          and_intro
            Top
            (And Top Top)
            Proved
            (and_intro Top Top Proved Proved)

        theorem cc5_group_tokens_preserved :
            Equal
              (List Char)
              (render_content (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc)
              (doc_content cc5_group_doc) =
          (proof preserves_text_tokens for render_content)
            (Suc (Suc (Suc (Suc Zero))))
            cc5_group_doc
            cc5_group_valid

        theorem cc5_group_width_independent :
            Equal
              (List Char)
              (render_content (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc)
              (render_content (Suc (Suc (Suc (Suc (Suc Zero))))) cc5_group_doc) =
          (proof width_independent for render_content)
            (Suc (Suc (Suc (Suc Zero))))
            (Suc (Suc (Suc (Suc (Suc Zero)))))
            cc5_group_doc
            cc5_group_valid

        theorem cc5_group_render_fixed_point :
            Equal
              (List Char)
              (render
                (Suc (Suc (Suc (Suc Zero))))
                (Text (render (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc)))
              (render (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc) =
          (proof fixed_point for render) (Suc (Suc (Suc (Suc Zero)))) cc5_group_doc
        "#,
    )
    .expect("proof-consumption probes must elaborate");
}

#[test]
fn ordered_dependency_closure_elaborates_transparent_pretty_doc() {
    let env = full_env();
    assert!(env.globals.contains_key("Doc"));
    for constructor in ["Text", "Line", "Concat", "Nest", "Group", "Alt"] {
        assert!(
            env.globals.contains_key(constructor),
            "expected checked Doc constructor `{constructor}`"
        );
    }
    assert_transparent_globals(
        &env,
        &[
            "doc_content",
            "DocContentInvariant",
            "pretty_repeat_char",
            "doc_flat_width",
            "doc_fits",
            "render_mode",
            "render",
            "render_content_mode",
            "render_content",
            "render_content_mode::preserves_text_tokens",
            "render_content::preserves_text_tokens",
            "render_content::width_independent",
            "render::fixed_point",
            "text_string",
            "render_string",
        ],
    );
}

#[test]
fn group_and_alt_flip_at_the_exact_fitting_boundary() {
    let mut env = full_env();
    add_render_probes(&mut env);
    let mut store = make_store(&env);

    for name in ["cc5_group_below", "cc5_alt_below"] {
        assert_eq!(
            as_text(&env, &eval_global(&env, &mut store, name)),
            "ab\n  cd"
        );
    }
    for name in [
        "cc5_group_boundary",
        "cc5_group_above",
        "cc5_alt_boundary",
        "cc5_alt_above",
    ] {
        assert_eq!(as_text(&env, &eval_global(&env, &mut store, name)), "ab cd");
    }

    let first = eval_global(&env, &mut store, "cc5_group_boundary");
    let second = eval_global(&env, &mut store, "cc5_group_boundary_again");
    assert_eq!(
        list_char_codepoints(&env, &first),
        list_char_codepoints(&env, &second),
        "same Doc and width must render byte-identically across runs"
    );
}

#[test]
fn all_three_laws_are_checked_and_consumable_as_proofs() {
    let mut env = full_env();
    add_render_probes(&mut env);
    add_law_probes(&mut env);
    assert_transparent_globals(
        &env,
        &[
            "cc5_group_tokens_preserved",
            "cc5_group_width_independent",
            "cc5_group_render_fixed_point",
        ],
    );
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the roots loader reports the exact six-name Doc surface; a real
/// selective client constructs `Text` from `List Char` and calls `text_string`
/// from `String`; the private `render_string` declaration remains owned,
/// transparent, and unimportable. CLAIMED: Doc retains its structural carrier
/// boundary and its internal opaque-String adapter without widening that API.
/// THE GAP: loader shape does not prove rendering behavior or laws, which the
/// three preceding tests exercise independently.
#[test]
fn pretty_doc_loader_surface_and_string_boundary_are_behavioral() {
    let expected = names(&["Concat", "Doc", "Group", "Line", "Text", "text_string"]);
    assert_eq!(
        catalog_publication::published_module_surfaces(
            PRETTY_DOC_KEN_MD,
            "Capability.Formatting.Doc",
            "cc5_pretty_doc",
        ),
        expected,
    );

    let mut env = dependency_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Formatting.Doc")
        .expect("Capability.Formatting.Doc must roots-load")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "CC5 must add zero trusted-base entries");

    env.elaborate_file(
        r#"
        import Capability.Formatting.Doc (Doc, Text, text_string)

        fn cc5_text_shape (characters : List Char) : Doc = Text characters

        fn cc5_text_string_shape (value : String) : Doc = text_string value
        "#,
    )
    .expect("selective client must consume Text and text_string at their intended types");

    let render_string = *env
        .globals
        .get("Capability.Formatting.Doc.render_string")
        .expect("Formatting.Doc must retain its owned private render_string declaration");
    assert!(
        owned.contains(&render_string) && env.env.transparent_body(render_string).is_some(),
        "render_string must remain an owned, checked private adapter"
    );
    let private_error = env
        .elaborate_file("import Capability.Formatting.Doc (render_string)")
        .expect_err("render_string must remain private");
    assert!(
        matches!(private_error, ken_elaborator::ElabError::UnboundName { ref name, .. }
            if name == "Capability.Formatting.Doc.render_string"),
        "private render_string probe must fail at its own qualified name, got {private_error:?}"
    );

    let references = catalog_or::owned_references(&env, &owned);
    let diagnostic_references = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            (name.starts_with("Capability.Diagnostics.Core.") && references.contains(id))
                .then(|| name.clone())
        })
        .collect::<BTreeSet<_>>();
    assert!(
        diagnostic_references.is_empty(),
        "Doc checked declarations must remain diagnostics-independent: {diagnostic_references:?}"
    );

    let equal = env.globals["Equal"];
    let string = env.globals["String"];
    let equal_string_owners = owned
        .iter()
        .filter(|id| {
            declaration_terms(
                env.env
                    .lookup(**id)
                    .unwrap_or_else(|| panic!("owned global {id:?} must resolve")),
            )
            .into_iter()
            .any(|term| contains_equal_string(term, equal, string))
        })
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(
        equal_string_owners.is_empty(),
        "verified CC5 declarations must not assert equality across opaque String: {equal_string_owners:?}"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: roots-loaded Doc bodies retain the canonical `length`, `add`, and
/// `leq_nat` provider identities, retain neither retired local helper, and add
/// no trust. CLAIMED: structural width and fitting reuse the canonical Nat/List
/// operations directly. THE GAP: the fitting-boundary and proof-consumption
/// tests above separately establish the behavior and laws of those references.
#[test]
fn cc5_reuses_canonical_nat_operations_with_zero_trust_delta() {
    let mut env = dependency_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.Formatting.Doc")
        .expect("Capability.Formatting.Doc must roots-load over its dependency fixture");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "CC5 must add zero trusted-base entries");

    let length = env.globals["Data.Collections.Derived.length"];
    let add = env.globals["Data.Numeric.Nat.Arithmetic.add"];
    let leq_nat = env.globals["Core.Classes.LawfulClasses.leq_nat"];
    assert!(env.env.transparent_body(length).is_some());
    assert!(env.env.transparent_body(add).is_some());
    assert!(env.env.transparent_body(leq_nat).is_some());
    for local in ["pretty_nat_add", "pretty_nat_leq"] {
        assert!(
            !env.globals
                .contains_key(&format!("Capability.Formatting.Doc.{local}")),
            "Doc must not mint local Nat operation `{local}`"
        );
    }

    for (name, provider) in [
        ("doc_flat_width", length),
        ("doc_flat_width", add),
        ("render_mode", add),
        ("doc_fits", leq_nat),
    ] {
        let qualified = format!("Capability.Formatting.Doc.{name}");
        let id = env.globals[&qualified];
        let body = match env.env.lookup(id) {
            Some(Decl::Transparent { body, .. }) => body,
            other => panic!("{qualified} must be transparent, got {other:?}"),
        };
        assert!(
            term_mentions(body, provider),
            "{qualified} must retain its canonical provider GlobalId"
        );
    }
}
