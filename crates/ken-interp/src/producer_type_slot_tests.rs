//! Evaluator-level C5 observations for checked producer result types. Each
//! positive compares the producer with a separately elaborated source value;
//! each negative differs in content through the same producer/type path.
use super::*;
use ken_elaborator::{ElabEnv, NumericLitVal};

fn fixture(decls: &[&str]) -> (ElabEnv, EvalStore) {
    let mut elab = ElabEnv::new().expect("checked prelude");
    for decl in decls {
        elab.elaborate_decl(decl)
            .unwrap_or_else(|err| panic!("{decl}: {err:?}"));
    }
    let mut store = EvalStore::new();
    for (id, literal) in &elab.num_values {
        let val = match literal {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Float(n) => EvalVal::Float(*n),
            NumericLitVal::Float32(n) => EvalVal::Float32(*n),
            NumericLitVal::Decimal { coeff, exp } => {
                decimal_value(elab.prelude_env.mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
            NumericLitVal::Bytes(bytes) => EvalVal::Bytes(bytes.clone()),
        };
        store.num_values.insert(*id, val);
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: elab.prelude_env.nil_id,
        cons_id: elab.prelude_env.cons_id,
    });
    (elab, store)
}

fn value(elab: &ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let (_, body) = elab
        .env
        .transparent_body(elab.globals[name])
        .expect("checked source body");
    eval(&[], &body, &elab.env, store)
}

/// Promise class: durable fail-closed authority boundary. MEASURED: an
/// unavailable checked result, wrong owner or wrong arity gives no type slot.
/// CLAIMED: the caller retains its existing Unknown/Neutral fallback rather
/// than treating an unrelated family as a List element. THE GAP: the helper
/// checks shape, not the slot's type; only checked producer terms may feed it.
/// Each producer row below tests an available authority independently.
#[test]
fn type_slots_require_the_checked_owner_and_exact_parameter_arity() {
    let (elab, mut store) = fixture(&[]);
    let ids = store.list_char_ids.clone().expect("List identities");
    let list = elab.prelude_env.list_id;
    let value = EvalVal::IndTypeApp {
        id: list,
        args: Rc::new(vec![EvalVal::Int(1)]),
    };
    assert_eq!(
        list_element_slot(elab.prelude_env.byte_length_id, &ids, &elab.env, &mut store),
        EvalVal::Unknown,
        "a checked Int result cannot supply a List element"
    );
    assert!(type_value_slots(&EvalVal::Unknown, list, &elab.env).is_none());
    assert!(type_value_slots(&value, elab.prelude_env.option_id, &elab.env).is_none());
    assert!(type_value_slots(
        &EvalVal::IndTypeApp {
            id: list,
            args: Rc::new(vec![])
        },
        list,
        &elab.env,
    )
    .is_none());
}

/// Promise class: durable invariant. MEASURED: typed producer and source list
/// structures agree; equal-content lists with different element types disagree.
/// CLAIMED: a missing or defaulted element slot cannot make C5 admit a false
/// equality. THE GAP: the five kernel-checked casts are a separate CLI row.
#[test]
fn checked_list_views_match_source_but_not_other_content_or_element_type() {
    let (elab, mut store) = fixture(&[
        "data CIBytes (xs : List UInt8) : Type where { MkCIBytes : CIBytes xs }",
        "const chars : List Char = string_to_list_char \"a\"",
        "const bytes : List UInt8 = bytes_to_list (bytes_encode \"a\")",
        "const otherBytes : List UInt8 = bytes_to_list (bytes_encode \"b\")",
        "const sourceBytes : List UInt8 = Cons UInt8 97 (Nil UInt8)",
    ]);
    let chars = value(&elab, &mut store, "chars");
    let bytes = value(&elab, &mut store, "bytes");
    let other = value(&elab, &mut store, "otherBytes");
    let source = value(&elab, &mut store, "sourceBytes");
    assert!(
        eq_type_eq(&bytes, &source),
        "view={bytes:?}, source={source:?}"
    );
    assert!(
        !eq_type_eq(&bytes, &other),
        "different byte content must disagree"
    );
    assert!(
        !eq_type_eq(&chars, &source),
        "Char vs UInt8 must disagree even with identical 97 contents"
    );

    let family = elab.globals["CIBytes"];
    let indexed = |index| EvalVal::IndTypeApp {
        id: family,
        args: Rc::new(vec![index]),
    };
    let from = indexed(bytes.clone());
    let to = indexed(source.clone());
    let proof = EvalVal::ReflVal {
        ty: Rc::new(from.clone()),
        val: Rc::new(EvalVal::Int(97)),
    };
    let ctor = make_ctor(elab.globals["MkCIBytes"], vec![bytes.clone()], &mut store);
    assert_eq!(
        cast_reduce(from.clone(), to.clone(), proof.clone(), ctor.clone()),
        ctor
    );
    let negative_from = indexed(other.clone());
    assert_eq!(
        cast_reduce(negative_from, to, proof, ctor),
        EvalVal::Unknown
    );
}

/// Promise class: durable C5 regularity for each checked safe-Bytes result
/// family. MEASURED: producer and source constructors agree at the type slots
/// and payload, but every changed payload/arm disagrees. CLAIMED: a shared
/// Neutral filler cannot stand for UInt8, Bytes, Utf8Error or String.
/// THE GAP: source declarations and primitive result terms are both checked by
/// the kernel; their value agreement is measured here rather than assumed.
#[test]
fn safe_bytes_option_and_result_slots_agree_with_source_values() {
    let (elab, mut store) = fixture(&[
        "const atProduced : Option UInt8 = bytes_at (bytes_encode \"a\") 0",
        "const atSource : Option UInt8 = Some UInt8 97",
        "const atDifferent : Option UInt8 = Some UInt8 98",
        "const sliceProduced : Option Bytes = bytes_slice (bytes_encode \"ab\") 0 1",
        "const sliceSource : Option Bytes = Some Bytes (bytes_encode \"a\")",
        "const sliceDifferent : Option Bytes = Some Bytes (bytes_encode \"b\")",
        "const decodeProduced : Result Utf8Error String = bytes_decode (bytes_encode \"a\")",
        "const decodeSource : Result Utf8Error String = Ok Utf8Error String \"a\"",
        "const decodeDifferent : Result Utf8Error String = Ok Utf8Error String \"b\"",
        "const decodeErrorSource : Result Utf8Error String = Err Utf8Error String InvalidUtf8",
    ]);
    let mut missed = Vec::new();
    for (produced, source, different) in [
        ("atProduced", "atSource", "atDifferent"),
        ("sliceProduced", "sliceSource", "sliceDifferent"),
        ("decodeProduced", "decodeSource", "decodeDifferent"),
    ] {
        let got = value(&elab, &mut store, produced);
        let expected = value(&elab, &mut store, source);
        let negative = value(&elab, &mut store, different);
        if !eq_type_eq(&got, &expected) {
            missed.push(format!("{produced}: producer={got:?}, source={expected:?}"));
        }
        assert!(
            !eq_type_eq(&got, &negative),
            "{produced}: changed content must disagree"
        );
    }
    let err = reduce_safe_bytes_primitive(
        elab.globals["bytes_decode"],
        "bytes_decode",
        &[EvalVal::Bytes(vec![0xff])],
        &elab.env,
        &mut store,
    );
    let err_source = value(&elab, &mut store, "decodeErrorSource");
    let ok_source = value(&elab, &mut store, "decodeSource");
    if !eq_type_eq(&err, &err_source) {
        missed.push(format!(
            "decode Err: producer={err:?}, source={err_source:?}"
        ));
    }
    assert!(
        !eq_type_eq(&err, &ok_source),
        "Err and Ok contents must disagree"
    );
    assert!(missed.is_empty(), "{}", missed.join("; "));
}

/// Promise class: durable C5 regularity for the directory and FileError path
/// producers. MEASURED: a real reified directory reply and a constructed
/// FileError path agree with checked source counterparts, and differing paths
/// disagree. CLAIMED: the producer uses the checked constructor field type,
/// not a guessed List or Option parameter. THE GAP: outer host Result slots
/// are still Unknown and intentionally excluded into a separate WP.
#[test]
fn fs_directory_and_file_error_path_slots_agree_with_source() {
    let (elab, mut store) = fixture(&[
        "const dirSource : List DirEntry = Cons DirEntry (MkDirEntry (bytes_encode \"a\") KFile) (Nil DirEntry)",
        "const dirDifferent : List DirEntry = Cons DirEntry (MkDirEntry (bytes_encode \"b\") KFile) (Nil DirEntry)",
        "const pathSource : Option Bytes = Some Bytes (bytes_encode \"a\")",
        "const pathDifferent : Option Bytes = Some Bytes (bytes_encode \"b\")",
    ]);
    let fs = FSIds::from_elab(&elab.prelude_env).expect("FS roles");
    let ids = ConsoleIds::from_elab(&elab.prelude_env).expect("host result roles");
    let request = ken_host::CanonicalRequestV1::FsReadDirectory {
        path: b".".to_vec(),
    };
    let reply = reify_host_reply_v1(
        ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::DirectoryEntries(vec![
            ken_host::DirEntryV1 {
                name: b"a".to_vec(),
                kind: ken_host::FsNodeKindV1::File,
            },
        ])),
        None,
        None,
        &request,
        fs.op_read_directory_id,
        &fs,
        &ids,
        &elab.env,
        &mut store,
    )
    .expect("directory reply reifies");
    let EvalVal::Ctor { args, .. } = reply else {
        panic!("directory Result must be a constructor")
    };
    let directory = args.get(2).expect("Result payload");
    let source = value(&elab, &mut store, "dirSource");
    let different = value(&elab, &mut store, "dirDifferent");
    assert!(
        eq_type_eq(directory, &source),
        "directory={directory:?}, source={source:?}"
    );
    assert!(
        !eq_type_eq(directory, &different),
        "different name must disagree"
    );

    let cause = make_ctor(ids.notfound_id, vec![], &mut store);
    let file = file_error_value(fs.op_read_file_id, b"a", cause, &fs, &elab.env, &mut store);
    let EvalVal::Ctor { args, .. } = &file else {
        panic!("FileError must be a constructor")
    };
    let path = args.get(1).expect("FileError path field");
    let source = value(&elab, &mut store, "pathSource");
    let different = value(&elab, &mut store, "pathDifferent");
    assert!(
        eq_type_eq(path, &source),
        "file path={path:?}, source={source:?}"
    );
    assert!(
        !eq_type_eq(path, &different),
        "different path must disagree"
    );
}
