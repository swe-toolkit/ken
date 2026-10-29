//! C5 host-response slots (spec/40-runtime/42-evaluation.md §3.6): checked
//! Vis families drive real CaptureHost replies. Promise class: durable
//! invariant. Entropy's unavailable random-bytes operation is a labelled
//! transition sentinel, retired when that operation is promoted.

use std::path::PathBuf;

use ken_elaborator::capabilities::{Cap, AUTH_FULL};
use ken_interp::{
    apply, eval, CaptureHost, ConsoleIds, ConsoleStream, CoproductIds, EvalStore, EvalVal, FSIds,
    PosixHost, RunIoError,
};
use ken_kernel::{Context, GlobalId, Term};

struct Fixture {
    elab: ken_elaborator::ElabEnv,
    ids: ConsoleIds,
    fs: FSIds,
    coproduct: CoproductIds,
    store: EvalStore,
}

impl Fixture {
    fn new() -> Self {
        let mut elab = ken_elaborator::ElabEnv::new().expect("checked prelude");
        elab.elaborate_decl(
            "fn rt_capture_reply (eff : Type) (resp : eff -> Type) \
             (op : eff) (reply : resp op) : ITree eff resp (resp op) = \
             ITree.Ret eff resp (resp op) reply",
        )
        .expect("checked first-response continuation");
        elab.elaborate_decl(
            "data RtHostIndex (e : Type) (a : Type) (r : Result e a) : Type \
             where { MkRtHostIndex : RtHostIndex e a r }",
        )
        .expect("checked response-index family");
        let ids = ConsoleIds::from_elab(&elab.prelude_env).expect("Console ABI");
        let fs = FSIds::from_elab(&elab.prelude_env).expect("FS ABI");
        let coproduct = CoproductIds {
            inl_id: elab.prelude_env.inl_id,
            inr_id: elab.prelude_env.inr_id,
        };
        let mut store = EvalStore::new();
        for (id, literal) in &elab.num_values {
            use ken_elaborator::NumericLitVal;
            let val = match literal {
                NumericLitVal::Int(v) => EvalVal::from(v.clone()),
                NumericLitVal::Float(v) => EvalVal::Float(*v),
                NumericLitVal::Float32(v) => EvalVal::Float32(*v),
                NumericLitVal::Decimal { coeff, exp } => ken_interp::decimal_value(
                    elab.prelude_env.mkdecimalpair_id,
                    coeff.clone(),
                    *exp,
                ),
                NumericLitVal::Str(v) => EvalVal::Str(v.clone()),
                NumericLitVal::Bytes(v) => EvalVal::Bytes(v.clone()),
            };
            store.num_values.insert(*id, val);
        }
        Self {
            elab,
            ids,
            fs,
            coproduct,
            store,
        }
    }

    fn id(&self, name: &str) -> GlobalId {
        *self
            .elab
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("missing {name}"))
    }

    fn value(&mut self, term: &Term) -> EvalVal {
        eval(&[], term, &self.elab.env, &mut self.store)
    }

    fn ty(&mut self, text: &str) -> Term {
        self.elab
            .elaborate_expr("RT-C5 checked type", text)
            .unwrap_or_else(|e| panic!("type `{text}` does not elaborate: {e:?}"))
            .0
    }

    fn eval_expr(&mut self, source: &str) -> EvalVal {
        let term = self.ty(source);
        self.value(&term)
    }

    fn named(&mut self, name: &str) -> EvalVal {
        self.value(&Term::const_(self.id(name), vec![]))
    }

    fn ctor(&mut self, name: &str) -> EvalVal {
        self.value(&Term::constructor(self.id(name), vec![]))
    }

    fn call(&mut self, name: &str, args: impl IntoIterator<Item = EvalVal>) -> EvalVal {
        let mut f = self.named(name);
        for arg in args {
            f = apply(f, arg, &self.elab.env, &mut self.store);
        }
        f
    }

    fn fs_action(&mut self, name: &str, args: impl IntoIterator<Item = EvalVal>) -> EvalVal {
        let mut all = vec![self.full(), self.cap()];
        all.extend(args);
        self.call(name, all)
    }

    fn construct(&mut self, name: &str, args: impl IntoIterator<Item = EvalVal>) -> EvalVal {
        let mut f = self.ctor(name);
        for arg in args {
            f = apply(f, arg, &self.elab.env, &mut self.store);
        }
        f
    }

    fn cap(&self) -> EvalVal {
        EvalVal::Cap(Cap::mint(AUTH_FULL, "FS"))
    }

    fn acquire_file_tree(&mut self) -> EvalVal {
        let cap = self.cap();
        self.acquire_file_with(cap, b"root/data")
    }

    fn acquire_file_with(&mut self, cap: EvalVal, path: &[u8]) -> EvalVal {
        let full = self.full();
        let unit = self.eval_expr("Unit");
        let mode = self.ctor("ResourceRead");
        // The bracket body is unreachable: capture_first replaces the first
        // Vis continuation. The op and response family are checked prelude.
        self.call(
            "withResource",
            [
                full,
                unit.clone(),
                unit,
                cap,
                Self::bytes(path),
                mode,
                EvalVal::Neutral,
            ],
        )
    }

    fn acquire_buffer_tree(&mut self, capacity: i64) -> EvalVal {
        let full = self.full();
        let unit = self.eval_expr("Unit");
        self.call(
            "withBuffer",
            [
                full,
                unit.clone(),
                unit,
                EvalVal::Int(capacity),
                EvalVal::Neutral,
            ],
        )
    }

    fn acquire_mapping_tree(&mut self, length: i64) -> EvalVal {
        let full = self.full();
        let unit = self.eval_expr("Unit");
        let source = self.construct("Anonymous", [EvalVal::Int(length)]);
        let protection = self.ctor("ReadOnly");
        self.call(
            "withMapping",
            [
                full,
                unit.clone(),
                unit,
                source,
                protection,
                EvalVal::Neutral,
            ],
        )
    }

    fn buffer_handle(&mut self, token: EvalVal) -> EvalVal {
        let id = self.elab.prelude_env.private_buffer_handle_id;
        let f = self.value(&Term::constructor(id, vec![]));
        let f = apply(f, token, &self.elab.env, &mut self.store);
        apply(f, EvalVal::Int(8), &self.elab.env, &mut self.store)
    }

    fn buffer_span(&mut self, token: EvalVal) -> EvalVal {
        let id = self.elab.prelude_env.private_buffer_span_id;
        let f = self.value(&Term::constructor(id, vec![]));
        let f = apply(f, token, &self.elab.env, &mut self.store);
        let f = apply(f, EvalVal::Int(0), &self.elab.env, &mut self.store);
        let zero = self.ctor("Zero");
        let length = self.construct("Suc", [zero]);
        apply(f, length, &self.elab.env, &mut self.store)
    }

    fn full(&mut self) -> EvalVal {
        self.ctor("AFull")
    }

    fn bytes(bytes: &[u8]) -> EvalVal {
        EvalVal::Bytes(bytes.to_vec())
    }

    // The op and response family come from a checked prelude Vis, not a
    // manufactured host reply. Replace only its tail with a kernel-checked
    // continuation so the exact host response becomes the observable result.
    fn capture_first(
        &mut self,
        tree: EvalVal,
        expected_ty: &Term,
        host: &mut CaptureHost,
    ) -> EvalVal {
        let EvalVal::Ctor { id, args, .. } = tree else {
            panic!("expected Vis")
        };
        assert_eq!(id, self.ids.vis_id, "expected a checked Vis node");
        let m = self.ids.params_len;
        let (effect, family, op) = (args[0].clone(), args[1].clone(), args[m].clone());
        assert!(
            !matches!(family, EvalVal::Unknown | EvalVal::Neutral),
            "reachable checked Vis has unavailable response family: {family:?}"
        );
        let selected = apply(family.clone(), op.clone(), &self.elab.env, &mut self.store);
        let declared = self.value(expected_ty);
        assert_eq!(
            selected, declared,
            "checked R(unpeeled op) differs from declared response type"
        );
        assert_eq!(
            ken_interp::eval::type_value_slots(&selected, self.id("Result"), &self.elab.env)
                .expect("the checked family selects Result")
                .len(),
            2,
            "Result must have two checked type slots"
        );
        let k = self.call(
            "rt_capture_reply",
            [effect.clone(), family.clone(), op.clone()],
        );
        let mut vis = self.ctor("ITree.Vis");
        for arg in [effect, family, selected, op, k] {
            vis = apply(vis, arg, &self.elab.env, &mut self.store);
        }
        ken_interp::run_io(
            vis,
            host,
            &self.ids,
            Some(&self.fs),
            None,
            Some(&self.coproduct),
            &self.elab.env,
            &mut self.store,
        )
        .expect("CaptureHost must dispatch and reify the checked Vis response")
    }

    fn reflexive_probe(&mut self, response: EvalVal, e: &str, a: &str) -> EvalVal {
        let e = self.ty(e);
        let a = self.ty(a);
        let result = Term::app(
            Term::app(Term::indformer(self.id("Result"), vec![]), e.clone()),
            a.clone(),
        );
        let index = Term::app(
            Term::app(
                Term::app(Term::indformer(self.id("RtHostIndex"), vec![]), e.clone()),
                a.clone(),
            ),
            Term::var(0),
        );
        let witness = Term::app(
            Term::app(
                Term::app(Term::constructor(self.id("MkRtHostIndex"), vec![]), e),
                a,
            ),
            Term::var(0),
        );
        let cast = Term::Cast(
            Box::new(index.clone()),
            Box::new(index.clone()),
            Box::new(Term::Refl(Box::new(index.clone()))),
            Box::new(witness),
        );
        let closed = Term::lam(result.clone(), cast);
        ken_kernel::check(
            &self.elab.env,
            &Context::new(),
            &closed,
            &Term::pi(result, index),
        )
        .expect("reflexive cast indexed on the host reply must kernel-check");
        let function = self.value(&closed);
        apply(function, response, &self.elab.env, &mut self.store)
    }

    fn alternative_error(&mut self, e: &str, actual: &EvalVal) -> EvalVal {
        match e {
            "FileError" => self.eval_expr("MkFileError OpReadFile (None Bytes) Unsupported"),
            "ResourceError" => {
                let alternative = if matches!(actual, EvalVal::Ctor { id, .. }
                    if *id == self.id("InvalidBounds"))
                {
                    "Closed"
                } else {
                    "InvalidBounds"
                };
                self.ctor(alternative)
            }
            "IOError" => {
                let alternative = if matches!(actual, EvalVal::Ctor { id, .. }
                    if *id == self.ids.brokenpipe_id)
                {
                    "InvalidInput"
                } else {
                    "BrokenPipe"
                };
                self.ctor(alternative)
            }
            _ => panic!("unknown checked error family {e}"),
        }
    }

    fn different_content(&mut self, e: &str, a: &str, ok: bool, actual: &EvalVal) -> EvalVal {
        if !ok {
            return self.alternative_error(e, actual);
        }
        match a {
            "Bytes" => match actual {
                EvalVal::Bytes(bytes) => {
                    let alternate = if bytes == b"different-content" {
                        b"other-content".as_slice()
                    } else {
                        b"different-content".as_slice()
                    };
                    Self::bytes(alternate)
                }
                other => panic!("Ok Bytes has wrong payload: {other:?}"),
            },
            "ReadResult" => {
                let EvalVal::Ctor { id, args, .. } = actual else {
                    panic!("expected checked Chunk payload: {actual:?}")
                };
                assert_eq!(*id, self.ids.chunk_id, "fixture read must be Chunk");
                let alternative = if args[0] == Self::bytes(b"different-content") {
                    b"other-content".as_slice()
                } else {
                    b"different-content".as_slice()
                };
                self.construct("Chunk", [Self::bytes(alternative)])
            }
            "FileMetadata" => {
                let EvalVal::Ctor { id, args, .. } = actual else {
                    panic!("expected FileMetadata: {actual:?}")
                };
                assert_eq!(*id, self.fs.mk_file_metadata_id);
                let size = if args[0] == EvalVal::Int(100) {
                    101
                } else {
                    100
                };
                self.construct("MkFileMetadata", [EvalVal::Int(size), args[1].clone()])
            }
            "(List DirEntry)" => {
                assert!(
                    matches!(actual, EvalVal::Ctor { id, .. }
                    if *id == self.fs.cons_id),
                    "fixture directory must be nonempty"
                );
                let dir_entry = self.eval_expr("DirEntry");
                self.construct("Nil", [dir_entry])
            }
            other => {
                panic!("Ok {other} has no same-constructor content negative; select a host Err row")
            }
        }
    }

    // MEASURED: for each checked response type, R(unpeeled op) equals an
    // independently elaborated source Result; its actual host response drives
    // a kernel-checked reflexive cast, whereas a different same-typed content
    // and the same payload under a different error-type slot do not agree.
    // CLAIMED: host Result type slots are derived from that Vis family, not
    // guessed from payloads. THE GAP: only reachable CaptureHost operations
    // in the enumerated family are rows; gated Entropy is tested separately.
    fn verify_pair(&mut self, tree: EvalVal, e: &str, a: &str, host: &mut CaptureHost) -> EvalVal {
        let expected_ty = self.ty(&format!("Result {e} {a}"));
        let response = self.capture_first(tree, &expected_ty, host);
        let EvalVal::Ctor { id, args, .. } = &response else {
            panic!("host Result must be constructor: {response:?}")
        };
        assert!(*id == self.ids.ok_id || *id == self.ids.err_id);
        assert_eq!(args.len(), 3, "Result slots + payload");
        let e_val = self.eval_expr(e);
        let a_val = self.eval_expr(a);
        let expected = self.construct(
            "MkRtHostIndex",
            [e_val.clone(), a_val.clone(), response.clone()],
        );
        let proof = self.reflexive_probe(response.clone(), e, a);
        assert_eq!(
            proof, expected,
            "cast must preserve the entire checked witness for {e}/{a}"
        );
        assert_eq!(args[0], e_val, "error slot from checked family");
        assert_eq!(args[1], a_val, "success slot from checked family");

        let is_ok = *id == self.ids.ok_id;
        let different = self.different_content(e, a, is_ok, &args[2]);
        assert_ne!(different, args[2], "negative must really change content");
        let ctor_name = if is_ok { "Ok" } else { "Err" };
        let alternate = self.construct(ctor_name, [e_val.clone(), a_val.clone(), different]);
        let EvalVal::Ctor {
            id: alternate_id,
            args: alternate_args,
            ..
        } = &alternate
        else {
            panic!("same-constructor alternate Result not a constructor: {alternate:?}")
        };
        assert_eq!(*alternate_id, *id, "only the Result payload may change");
        assert_eq!(alternate_args[0], args[0], "error type held fixed");
        assert_eq!(alternate_args[1], args[1], "success type held fixed");
        assert_ne!(alternate_args[2], args[2], "payload content must change");
        let alternate_witness = self.construct(
            "MkRtHostIndex",
            [e_val.clone(), a_val.clone(), alternate.clone()],
        );
        let alternate_cast = self.reflexive_probe(alternate.clone(), e, a);
        assert_eq!(
            alternate_cast, alternate_witness,
            "alternate cast must preserve changed content of {e}/{a}"
        );
        assert_ne!(
            proof, alternate_cast,
            "checked index must distinguish changed content"
        );

        let wrong_e = if e == "FileError" {
            "ResourceError"
        } else {
            "FileError"
        };
        let wrong_e_val = self.eval_expr(wrong_e);
        let ctor_name = if *id == self.ids.ok_id { "Ok" } else { "Err" };
        let wrong = self.construct(ctor_name, [wrong_e_val, a_val, args[2].clone()]);
        let EvalVal::Ctor {
            args: wrong_args, ..
        } = &wrong
        else {
            unreachable!()
        };
        assert_eq!(
            args[2], wrong_args[2],
            "control must keep payload identical"
        );
        assert_ne!(
            response, wrong,
            "same payload with another error type is false"
        );
        let wrong_cast = self.reflexive_probe(wrong, e, a);
        assert_ne!(
            proof, wrong_cast,
            "different error slot must change the index"
        );
        response
    }
}

fn host() -> CaptureHost {
    let mut host = CaptureHost::new(b"ambient-first".to_vec());
    host.insert_directory(b"root".to_vec());
    host.insert_file(b"root/data".to_vec(), b"file-first".to_vec());
    host
}

fn acquired_file(f: &mut Fixture, host: &mut CaptureHost) -> EvalVal {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cap = EvalVal::Cap(PosixHost::new_at(root).mint_fs_cap(AUTH_FULL));
    let tree = f.acquire_file_with(cap, b"conformance/fs/fixtures/three-lines.txt");
    let ty = f.ty("Result FileError (Resource ResourceKind.FsHandle)");
    let response = f.capture_first(tree, &ty, host);
    // ResourceToken identity is outside eq_type_eq's comparable ground cases.
    // Only extract this prerequisite; success slots have separate controls.
    resource_token(&response, &f.ids)
}

fn acquired_buffer(f: &mut Fixture, host: &mut CaptureHost) -> EvalVal {
    let tree = f.acquire_buffer_tree(8);
    let ty = f.ty("Result ResourceError (Resource ResourceKind.Buffer)");
    let response = f.capture_first(tree, &ty, host);
    resource_token(&response, &f.ids)
}

fn resource_token(result: &EvalVal, ids: &ConsoleIds) -> EvalVal {
    match result {
        EvalVal::Ctor { id, args, .. } if *id == ids.ok_id => match &args[2] {
            token @ EvalVal::ResourceToken(_) => token.clone(),
            other => panic!("acquire did not yield a resource token: {other:?}"),
        },
        other => panic!("acquire did not succeed: {other:?}"),
    }
}

fn assert_resource_success_slots(f: &mut Fixture, result: &EvalVal, e: &str, a: &str) {
    resource_token(result, &f.ids);
    let EvalVal::Ctor { args, .. } = result else {
        unreachable!()
    };
    assert_eq!(args[0], f.eval_expr(e), "successful resource error slot");
    assert_eq!(
        args[1],
        f.eval_expr(a),
        "successful resource payload type slot"
    );
}

#[test]
fn fs_file_error_bytes_plain_fs() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.fs_action("read_bytes", [Fixture::bytes(b"root/data")]);
    let response = f.verify_pair(tree, "FileError", "Bytes", &mut host);
    assert!(matches!(&response, EvalVal::Ctor { id, args, .. }
        if *id == f.ids.ok_id && matches!(args.get(2), Some(EvalVal::Bytes(b)) if b == b"file-first")));
}

#[test]
fn fs_file_error_bytes_hostio_inl() {
    let mut f = Fixture::new();
    let mut host = host();
    let action = f.fs_action("read_bytes", [Fixture::bytes(b"root/data")]);
    let response_ty = f.eval_expr("Result FileError Bytes");
    let full = f.full();
    let injected = f.call("host_fs", [full, response_ty, action]);
    let EvalVal::Ctor { args, .. } = &injected else {
        panic!("expected HostIO Vis")
    };
    assert!(
        matches!(&args[f.ids.params_len], EvalVal::Ctor { id, .. }
        if *id == f.coproduct.inl_id),
        "HostIO read must dispatch unpeeled InL"
    );
    let response = f.verify_pair(injected, "FileError", "Bytes", &mut host);
    assert!(matches!(&response, EvalVal::Ctor { id, args, .. }
        if *id == f.ids.ok_id && matches!(args.get(2), Some(EvalVal::Bytes(b)) if b == b"file-first")));
}

#[test]
fn fs_file_error_unit() {
    let mut f = Fixture::new();
    let mut host = host();
    let policy = f.ctor("CreateNew");
    let tree = f.fs_action(
        "write_file",
        [
            Fixture::bytes(b"root/data"),
            policy,
            Fixture::bytes(b"written"),
        ],
    );
    let response = f.verify_pair(tree, "FileError", "Unit", &mut host);
    assert!(
        matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id),
        "existing path must give a host Err with another FileError as negative"
    );
}

#[test]
fn fs_file_error_unit_preflight_error_without_host_call() {
    let mut f = Fixture::new();
    let mut host = host();
    let before = host.fs_trace().len();
    let preflight = f.fs_action(
        "change_mode",
        [Fixture::bytes(b"root/data"), EvalVal::Int(-1)],
    );
    let error = f.verify_pair(preflight, "FileError", "Unit", &mut host);
    assert!(matches!(error, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
    assert_eq!(
        host.fs_trace().len(),
        before,
        "preflight must not reach the host"
    );
}

#[test]
fn fs_file_error_file_metadata() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.fs_action("file_metadata", [Fixture::bytes(b"root/data")]);
    let response = f.verify_pair(tree, "FileError", "FileMetadata", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.ok_id));
}

#[test]
fn fs_file_error_directory_entries() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.fs_action("read_directory", [Fixture::bytes(b"root")]);
    let response = f.verify_pair(tree, "FileError", "(List DirEntry)", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.ok_id));
}

#[test]
fn fs_file_error_resource_fs_handle() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.acquire_file_tree();
    let response = f.verify_pair(
        tree,
        "FileError",
        "(Resource ResourceKind.FsHandle)",
        &mut host,
    );
    assert!(
        matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id),
        "CaptureHost's FS resource open refuses non-Posix virtual capabilities"
    );
}

#[test]
fn fs_resource_error_resource_buffer() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.acquire_buffer_tree(i64::MAX);
    let response = f.verify_pair(
        tree,
        "ResourceError",
        "(Resource ResourceKind.Buffer)",
        &mut host,
    );
    assert!(
        matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id),
        "an over-limit BufferAllocate must be refused by host dispatch"
    );
}

#[test]
fn fs_resource_error_resource_mapping() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.acquire_mapping_tree(i64::MAX);
    let response = f.verify_pair(
        tree,
        "ResourceError",
        "(Resource ResourceKind.Mapping)",
        &mut host,
    );
    assert!(
        matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id),
        "an over-limit MappingAllocate must be refused by host dispatch"
    );
}

#[test]
fn fs_successful_file_token_has_checked_slots() {
    let mut f = Fixture::new();
    let mut host = host();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let cap = EvalVal::Cap(PosixHost::new_at(root).mint_fs_cap(AUTH_FULL));
    let tree = f.acquire_file_with(cap, b"conformance/fs/fixtures/three-lines.txt");
    let ty = f.ty("Result FileError (Resource ResourceKind.FsHandle)");
    let response = f.capture_first(tree, &ty, &mut host);
    assert_resource_success_slots(
        &mut f,
        &response,
        "FileError",
        "(Resource ResourceKind.FsHandle)",
    );
}

#[test]
fn fs_successful_buffer_token_has_checked_slots() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.acquire_buffer_tree(8);
    let ty = f.ty("Result ResourceError (Resource ResourceKind.Buffer)");
    let response = f.capture_first(tree, &ty, &mut host);
    assert_resource_success_slots(
        &mut f,
        &response,
        "ResourceError",
        "(Resource ResourceKind.Buffer)",
    );
}

#[test]
fn fs_successful_mapping_token_has_checked_slots() {
    let mut f = Fixture::new();
    let mut host = host();
    let tree = f.acquire_mapping_tree(8);
    let ty = f.ty("Result ResourceError (Resource ResourceKind.Mapping)");
    let response = f.capture_first(tree, &ty, &mut host);
    assert_resource_success_slots(
        &mut f,
        &response,
        "ResourceError",
        "(Resource ResourceKind.Mapping)",
    );
    // A resource token is opaque and is not in eq_type_eq's comparable
    // ground-value cases; do not mislabel its cast as this slot test.
}

#[test]
fn fs_resource_error_file_metadata() {
    let mut f = Fixture::new();
    let mut host = host();
    let file = acquired_file(&mut f, &mut host);
    let full = f.full();
    let tree = f.call("resourceMetadata", [full, file]);
    let response = f.verify_pair(tree, "ResourceError", "FileMetadata", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
}

#[test]
fn fs_resource_error_unit() {
    let mut f = Fixture::new();
    let mut host = host();
    let file = acquired_file(&mut f, &mut host);
    let full = f.full();
    let tree = f.call("release", [full, file]);
    let response = f.verify_pair(tree, "ResourceError", "Unit", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
}

#[test]
fn fs_resource_error_read_progress() {
    let mut f = Fixture::new();
    let mut host = host();
    let file = acquired_file(&mut f, &mut host);
    let buffer = acquired_buffer(&mut f, &mut host);
    let buffer = f.buffer_handle(buffer);
    let window = f.construct("MkBufferWindow", [EvalVal::Int(0), EvalVal::Int(1)]);
    let full = f.full();
    let tree = f.call("readAt", [full, file, EvalVal::Int(0), buffer, window]);
    let response = f.verify_pair(tree, "ResourceError", "ReadProgress", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
}

#[test]
fn fs_resource_error_write_progress() {
    let mut f = Fixture::new();
    let mut host = host();
    let file = acquired_file(&mut f, &mut host);
    let buffer = acquired_buffer(&mut f, &mut host);
    let handle = f.buffer_handle(buffer.clone());
    let span = f.buffer_span(buffer);
    let full = f.full();
    let tree = f.call("writeAt", [full, file, EvalVal::Int(0), handle, span]);
    let response = f.verify_pair(tree, "ResourceError", "WriteProgress", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
}

#[test]
fn fs_resource_error_bytes() {
    let mut f = Fixture::new();
    let mut host = host();
    let buffer = acquired_buffer(&mut f, &mut host);
    let handle = f.buffer_handle(buffer.clone());
    let span = f.buffer_span(buffer);
    let full = f.full();
    let tree = f.call("freeze", [full, handle, span]);
    let response = f.verify_pair(tree, "ResourceError", "Bytes", &mut host);
    assert!(matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id));
}

#[test]
fn ambient_io_error_read_result_plain_io() {
    let mut f = Fixture::new();
    let stdin = f.ctor("Stdin");
    let action = f.call("read", [stdin, EvalVal::Int(20)]);
    let mut host = host();
    let response = f.verify_pair(action, "IOError", "ReadResult", &mut host);
    assert!(matches!(&response, EvalVal::Ctor { id, args, .. }
        if *id == f.ids.ok_id && matches!(&args[2], EvalVal::Ctor { id, args, .. }
            if *id == f.ids.chunk_id && matches!(&args[0], EvalVal::Bytes(bytes)
                if bytes == b"ambient-first"))));
}

#[test]
fn ambient_io_error_read_result_host_console() {
    let mut f = Fixture::new();
    let stdin = f.ctor("Stdin");
    let action = f.call("read", [stdin, EvalVal::Int(20)]);
    let full = f.full();
    let ty = f.eval_expr("Result IOError ReadResult");
    let injected = f.call("host_console", [full, ty, action]);
    let EvalVal::Ctor { args, .. } = &injected else {
        panic!("expected HostIO Vis")
    };
    let EvalVal::Ctor {
        id, args: inner, ..
    } = &args[f.ids.params_len]
    else {
        panic!("HostIO console op is not a coproduct")
    };
    assert_eq!(*id, f.coproduct.inr_id);
    assert!(
        matches!(&inner[2], EvalVal::Ctor { id, .. }
        if *id == f.coproduct.inl_id),
        "host_console op must be InR(InL(Read))"
    );
    let mut host = host();
    let response = f.verify_pair(injected, "IOError", "ReadResult", &mut host);
    assert!(matches!(&response, EvalVal::Ctor { id, args, .. }
        if *id == f.ids.ok_id && matches!(&args[2], EvalVal::Ctor { id, args, .. }
            if *id == f.ids.chunk_id && matches!(&args[0], EvalVal::Bytes(bytes)
                if bytes == b"ambient-first"))));
}

#[test]
fn ambient_io_error_unit() {
    let mut f = Fixture::new();
    let stdout = f.ctor("Stdout");
    let tree = f.call("write", [stdout, Fixture::bytes(b"written")]);
    let mut host = host();
    host.close(ConsoleStream::Stdout);
    let response = f.verify_pair(tree, "IOError", "Unit", &mut host);
    assert!(
        matches!(response, EvalVal::Ctor { id, .. } if id == f.ids.err_id),
        "closed stream must give BrokenPipe, not a Unit success"
    );
    assert!(host.stdout().is_empty());
}

/// Transition sentinel: EntropyRandomBytes is RepresentedUnavailable. When
/// promoted, retire this refusal and add its IOError/Bytes response row.
#[test]
fn entropy_random_bytes_stays_unavailable_until_promotion() {
    let mut f = Fixture::new();
    let action = f.call("random_bytes", [EvalVal::Int(8)]);
    let full = f.full();
    let result_ty = f.eval_expr("Result IOError Bytes");
    let tree = f.call("host_entropy", [full, result_ty, action]);
    let mut host = host();
    let before = (host.trace().len(), host.fs_trace().len());
    assert!(
        matches!(
            ken_interp::run_io(
                tree,
                &mut host,
                &f.ids,
                Some(&f.fs),
                None,
                Some(&f.coproduct),
                &f.elab.env,
                &mut f.store,
            ),
            Err(RunIoError::UnknownEffect(_))
        ),
        "unavailable entropy must not invent an IOError/Bytes host reply"
    );
    assert_eq!((host.trace().len(), host.fs_trace().len()), before);
}
