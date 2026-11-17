//! Independently authored kernel propositions for the private Resource bracket laws.
//! No law type or body is inspected here. The caller selects checked private
//! helper identities from their owner laws' left heads and supplies them.

use ken_elaborator::ElabEnv;
use ken_kernel::{subst::weaken, GlobalId, Term};

#[derive(Clone, Copy)]
pub struct Helpers {
    pub after_open: GlobalId,
    pub after_buffer: GlobalId,
    pub after_mapping: GlobalId,
    pub release_if_live: GlobalId,
    pub settle_result: GlobalId,
    pub mapping_handle: GlobalId,
}

struct Kit<'a> {
    env: &'a ElabEnv,
    helpers: Helpers,
}

#[derive(Clone, Default)]
struct Binders(Vec<(&'static str, Term)>);

impl Binders {
    fn add(&mut self, name: &'static str, ty: Term) {
        assert!(!self.0.iter().any(|(existing, _)| *existing == name));
        self.0.push((name, ty));
    }

    fn v(&self, name: &str) -> Term {
        Term::var(
            self.0.len()
                - 1
                - self
                    .0
                    .iter()
                    .position(|(existing, _)| *existing == name)
                    .unwrap_or_else(|| panic!("unbound bracket client variable `{name}`")),
        )
    }

    fn finish(self, conclusion: Term) -> Term {
        self.0
            .into_iter()
            .rev()
            .fold(conclusion, |body, (_, domain)| Term::pi(domain, body))
    }
}

impl Kit<'_> {
    fn id(&self, name: &str) -> GlobalId {
        *self
            .env
            .globals
            .get(name)
            .unwrap_or_else(|| panic!("checked `{name}` is absent"))
    }

    fn public(&self, name: &str) -> Term {
        let id = self.id(name);
        if self.env.env.inductive(id).is_some() {
            Term::indformer(id, vec![])
        } else if self.env.env.constructor(id).is_some() {
            Term::constructor(id, vec![])
        } else {
            Term::const_(id, vec![])
        }
    }

    fn private(&self, id: GlobalId) -> Term {
        Term::const_(id, vec![])
    }

    fn ctor(&self, id: GlobalId) -> Term {
        Term::constructor(id, vec![])
    }

    fn app(&self, head: Term, args: impl IntoIterator<Item = Term>) -> Term {
        args.into_iter().fold(head, Term::app)
    }

    fn call(&self, name: &str, args: impl IntoIterator<Item = Term>) -> Term {
        self.app(self.public(name), args)
    }

    fn resource(&self, kind: Term) -> Term {
        self.app(self.private(self.env.prelude_env.resource_id), [kind])
    }

    fn result(&self, err: Term, ok: Term) -> Term {
        self.call("Result", [err, ok])
    }

    fn bracket(&self, e: Term, r: Term) -> Term {
        self.call("ResourceBracketResult", [e, r])
    }

    fn body_result(&self, e: Term, r: Term) -> Term {
        self.call("ResourceBodyResult", [e, r])
    }

    fn io(&self, a: Term, result: Term) -> Term {
        self.call("HostIO", [a, result])
    }

    fn fsop(&self, a: Term) -> Term {
        self.app(
            Term::indformer(self.env.prelude_env.runtime_roles.fs_family, vec![]),
            [a],
        )
    }

    fn ambient(&self) -> Term {
        self.public("AmbientOp")
    }

    fn operations(&self, a: Term) -> Term {
        self.app(
            Term::indformer(self.env.prelude_env.coproduct_id, vec![]),
            [self.fsop(a), self.ambient()],
        )
    }

    fn response(&self, a: Term) -> Term {
        let fs = self.fsop(a.clone());
        let ambient = self.ambient();
        let fs_resp = self.app(self.public("fs_resp"), [a]);
        self.app(
            self.private(self.env.prelude_env.resp_coproduct_id),
            [fs, ambient, fs_resp, self.public("ambient_resp")],
        )
    }

    fn inl(&self, a: Term, op: Term) -> Term {
        self.app(
            self.ctor(self.env.prelude_env.inl_id),
            [self.fsop(a), self.ambient(), op],
        )
    }

    fn vis(&self, a: Term, result: Term, op: Term, next: Term) -> Term {
        self.app(
            self.ctor(self.env.prelude_env.vis_id),
            [
                self.operations(a.clone()),
                self.response(a),
                result,
                op,
                next,
            ],
        )
    }

    fn ret(&self, a: Term, result: Term, value: Term) -> Term {
        self.app(
            self.ctor(self.env.prelude_env.ret_id),
            [self.operations(a.clone()), self.response(a), result, value],
        )
    }

    fn bind(&self, a: Term, first: Term, result: Term, tree: Term, next: Term) -> Term {
        self.app(
            self.private(self.env.prelude_env.bind_id),
            [
                self.operations(a.clone()),
                self.response(a),
                first,
                result,
                tree,
                next,
            ],
        )
    }

    fn equal(&self, carrier: Term, lhs: Term, rhs: Term) -> Term {
        self.call("Equal", [carrier, lhs, rhs])
    }

    fn arrow(&self, domain: Term, codomain: Term) -> Term {
        Term::pi(domain, weaken(&codomain, 1))
    }

    fn body(&self, b: &mut Binders, resource: Term) {
        let body_result = self.body_result(b.v("e"), b.v("r"));
        b.add("body", self.arrow(resource, self.io(b.v("a"), body_result)));
    }

    fn error(&self, error: Term, value: Term, err: Term) -> Term {
        self.call("Err", [error, value, err])
    }

    fn success(&self, error: Term, value: Term, ok: Term) -> Term {
        self.call("Ok", [error, value, ok])
    }

    fn base(&self) -> Binders {
        let mut b = Binders::default();
        b.add("a", self.public("Auth"));
        b.add("e", Term::Type(ken_kernel::Level::zero()));
        b.add("r", Term::Type(ken_kernel::Level::zero()));
        b
    }

    fn expected(&self, name: &str) -> (Term, Term, &'static str) {
        let mut b = self.base();
        let fs = self.public("ResourceKind.FsHandle");
        let buffer = self.public("ResourceKind.Buffer");
        let mapping = self.public("ResourceKind.Mapping");
        let file_error = self.public("FileError");
        let resource_error = self.public("ResourceError");
        let zero = Term::IntLit(0.into());
        let mut mutant = None;
        let (error, lhs, rhs) = match name {
            "resource_with_acquire" => {
                b.add("cap", self.call("Cap", [b.v("a")]));
                b.add("path", self.public("Bytes"));
                b.add("mode", self.public("ResourceOpenMode"));
                self.body(&mut b, self.resource(fs.clone()));
                let args = [
                    b.v("a"),
                    b.v("e"),
                    b.v("r"),
                    b.v("cap"),
                    b.v("path"),
                    b.v("mode"),
                    b.v("body"),
                ];
                let left = self.call("withResource", args);
                let op = |mode| {
                    self.inl(
                        b.v("a"),
                        self.app(
                            self.ctor(self.env.prelude_env.private_fs_open_id),
                            [b.v("a"), b.v("cap"), b.v("path"), mode],
                        ),
                    )
                };
                let next = self.app(
                    self.private(self.helpers.after_open),
                    [b.v("a"), b.v("e"), b.v("r"), b.v("body")],
                );
                let ty = self.result(file_error.clone(), self.bracket(b.v("e"), b.v("r")));
                mutant = Some((
                    self.vis(
                        b.v("a"),
                        ty.clone(),
                        op(self.public("ResourceOpenMode.ResourceRead")),
                        next.clone(),
                    ),
                    "Vis PrivateFsOpen mode replaced by ResourceRead",
                ));
                (
                    file_error,
                    left,
                    self.vis(b.v("a"), ty, op(b.v("mode")), next),
                )
            }
            "resource_after_open_error" | "resource_after_open_success" => {
                self.body(&mut b, self.resource(fs.clone()));
                let is_error = name.ends_with("error");
                if is_error {
                    b.add("open_error", file_error.clone());
                } else {
                    b.add("resource", self.resource(fs.clone()));
                }
                let outcome = if is_error {
                    self.error(file_error.clone(), self.resource(fs), b.v("open_error"))
                } else {
                    self.success(file_error.clone(), self.resource(fs), b.v("resource"))
                };
                let lhs = self.app(
                    self.private(self.helpers.after_open),
                    [b.v("a"), b.v("e"), b.v("r"), b.v("body"), outcome],
                );
                let result = self.result(file_error.clone(), self.bracket(b.v("e"), b.v("r")));
                let rhs = if is_error {
                    let payload = self.error(
                        file_error.clone(),
                        self.bracket(b.v("e"), b.v("r")),
                        b.v("open_error"),
                    );
                    self.ret(b.v("a"), result.clone(), payload)
                } else {
                    let body = self.app(b.v("body"), [b.v("resource")]);
                    let release = self.app(
                        self.private(self.helpers.release_if_live),
                        [
                            b.v("a"),
                            file_error.clone(),
                            b.v("e"),
                            b.v("r"),
                            self.public("ResourceKind.FsHandle"),
                            b.v("resource"),
                        ],
                    );
                    self.bind(
                        b.v("a"),
                        self.body_result(b.v("e"), b.v("r")),
                        result.clone(),
                        body,
                        release,
                    )
                };
                mutant = Some(if is_error {
                    let not_found = self.call(
                        "MkFileError",
                        [
                            self.public("FileOperation.OpReadFile"),
                            self.call("None", [self.public("Bytes")]),
                            self.public("IOError.NotFound"),
                        ],
                    );
                    let payload = self.error(
                        file_error.clone(),
                        self.bracket(b.v("e"), b.v("r")),
                        not_found,
                    );
                    (
                        self.ret(b.v("a"), result.clone(), payload),
                        "Ret Err open_error replaced by checked NotFound FileError",
                    )
                } else {
                    // The genuine success path binds a release effect. This
                    // well-typed rival settles Closed immediately instead.
                    let mut cont = b.clone();
                    let response = self.body_result(b.v("e"), b.v("r"));
                    cont.add("settled_body", response.clone());
                    let settled_closed = self.error(
                        resource_error.clone(),
                        self.public("Unit"),
                        self.public("ResourceError.Closed"),
                    );
                    let value = self.app(
                        self.private(self.helpers.settle_result),
                        [
                            file_error.clone(),
                            cont.v("e"),
                            cont.v("r"),
                            cont.v("settled_body"),
                            settled_closed,
                        ],
                    );
                    let next = Term::lam(
                        response.clone(),
                        self.ret(
                            cont.v("a"),
                            self.result(file_error.clone(), self.bracket(cont.v("e"), cont.v("r"))),
                            value,
                        ),
                    );
                    (
                        self.bind(
                            b.v("a"),
                            response,
                            result.clone(),
                            self.app(b.v("body"), [b.v("resource")]),
                            next,
                        ),
                        "Bind release continuation replaced by immediate Closed settlement",
                    )
                });
                (file_error, lhs, rhs)
            }
            "buffer_with_acquire"
            | "buffer_after_allocate_error"
            | "buffer_after_allocate_success" => {
                b.add("capacity", self.public("Int"));
                self.body(&mut b, self.public("BufferHandle"));
                let acquire = name == "buffer_with_acquire";
                let failure = name.ends_with("error");
                if failure {
                    b.add("allocate_error", resource_error.clone());
                } else if !acquire {
                    b.add("resource", self.resource(buffer.clone()));
                }
                let result = self.result(resource_error.clone(), self.bracket(b.v("e"), b.v("r")));
                if acquire {
                    let left = self.call(
                        "withBuffer",
                        [b.v("a"), b.v("e"), b.v("r"), b.v("capacity"), b.v("body")],
                    );
                    let op = |capacity| {
                        self.inl(
                            b.v("a"),
                            self.app(
                                self.ctor(self.env.prelude_env.private_buffer_allocate_id),
                                [b.v("a"), capacity],
                            ),
                        )
                    };
                    let next = self.app(
                        self.private(self.helpers.after_buffer),
                        [b.v("a"), b.v("e"), b.v("r"), b.v("capacity"), b.v("body")],
                    );
                    mutant = Some((
                        self.vis(b.v("a"), result.clone(), op(zero.clone()), next.clone()),
                        "Vis PrivateBufferAllocate capacity replaced by zero",
                    ));
                    (
                        resource_error,
                        left,
                        self.vis(b.v("a"), result, op(b.v("capacity")), next),
                    )
                } else {
                    let outcome = if failure {
                        self.error(
                            resource_error.clone(),
                            self.resource(buffer.clone()),
                            b.v("allocate_error"),
                        )
                    } else {
                        self.success(
                            resource_error.clone(),
                            self.resource(buffer.clone()),
                            b.v("resource"),
                        )
                    };
                    let left = self.app(
                        self.private(self.helpers.after_buffer),
                        [
                            b.v("a"),
                            b.v("e"),
                            b.v("r"),
                            b.v("capacity"),
                            b.v("body"),
                            outcome,
                        ],
                    );
                    let right = if failure {
                        let err = |error| {
                            self.error(
                                resource_error.clone(),
                                self.bracket(b.v("e"), b.v("r")),
                                error,
                            )
                        };
                        mutant = Some((
                            self.ret(
                                b.v("a"),
                                result.clone(),
                                err(self.public("ResourceError.InvalidBounds")),
                            ),
                            "Ret Err allocate_error replaced by InvalidBounds",
                        ));
                        self.ret(b.v("a"), result.clone(), err(b.v("allocate_error")))
                    } else {
                        let handle = |capacity| {
                            self.app(
                                self.ctor(self.env.prelude_env.private_buffer_handle_id),
                                [b.v("resource"), capacity],
                            )
                        };
                        let release = self.app(
                            self.private(self.helpers.release_if_live),
                            [
                                b.v("a"),
                                resource_error.clone(),
                                b.v("e"),
                                b.v("r"),
                                buffer,
                                b.v("resource"),
                            ],
                        );
                        let bind = |capacity| {
                            self.bind(
                                b.v("a"),
                                self.body_result(b.v("e"), b.v("r")),
                                result.clone(),
                                self.app(b.v("body"), [handle(capacity)]),
                                release.clone(),
                            )
                        };
                        mutant = Some((
                            bind(zero.clone()),
                            "Bind body PrivateBufferHandle capacity replaced by zero",
                        ));
                        bind(b.v("capacity"))
                    };
                    (resource_error, left, right)
                }
            }
            "mapping_with_anonymous_acquire"
            | "mapping_with_file_acquire"
            | "mapping_after_allocate_error"
            | "mapping_after_allocate_success" => {
                let file = name == "mapping_with_file_acquire";
                let acquire = name.starts_with("mapping_with_");
                if file {
                    b.add("file", self.resource(fs.clone()));
                }
                b.add("length", self.public("Int"));
                if acquire {
                    b.add("protection", self.public("MappingProt"));
                }
                self.body(&mut b, self.public("MappingHandle"));
                let failure = name.ends_with("error");
                if failure {
                    b.add("allocate_error", resource_error.clone());
                } else if !acquire {
                    b.add("resource", self.resource(mapping.clone()));
                }
                let result = self.result(resource_error.clone(), self.bracket(b.v("e"), b.v("r")));
                if acquire {
                    let source = if file {
                        self.call("FileBacked", [b.v("file"), b.v("length")])
                    } else {
                        self.call("Anonymous", [b.v("length")])
                    };
                    let left = self.call(
                        "withMapping",
                        [
                            b.v("a"),
                            b.v("e"),
                            b.v("r"),
                            source,
                            b.v("protection"),
                            b.v("body"),
                        ],
                    );
                    let op = |length| {
                        self.inl(
                            b.v("a"),
                            if file {
                                self.app(
                                    self.ctor(self.env.prelude_env.private_mapping_acquire_file_id),
                                    [b.v("a"), b.v("file"), length, b.v("protection")],
                                )
                            } else {
                                self.app(
                                    self.ctor(self.env.prelude_env.private_mapping_allocate_id),
                                    [b.v("a"), length, b.v("protection")],
                                )
                            },
                        )
                    };
                    let next = self.app(
                        self.private(self.helpers.after_mapping),
                        [b.v("a"), b.v("e"), b.v("r"), b.v("length"), b.v("body")],
                    );
                    mutant = Some((
                        self.vis(b.v("a"), result.clone(), op(zero.clone()), next.clone()),
                        if file {
                            "Vis PrivateMappingAcquireFile length replaced by zero"
                        } else {
                            "Vis PrivateMappingAllocate length replaced by zero"
                        },
                    ));
                    (
                        resource_error,
                        left,
                        self.vis(b.v("a"), result, op(b.v("length")), next),
                    )
                } else {
                    let outcome = if failure {
                        self.error(
                            resource_error.clone(),
                            self.resource(mapping.clone()),
                            b.v("allocate_error"),
                        )
                    } else {
                        self.success(
                            resource_error.clone(),
                            self.resource(mapping.clone()),
                            b.v("resource"),
                        )
                    };
                    let left = self.app(
                        self.private(self.helpers.after_mapping),
                        [
                            b.v("a"),
                            b.v("e"),
                            b.v("r"),
                            b.v("length"),
                            b.v("body"),
                            outcome,
                        ],
                    );
                    let right = if failure {
                        let err = |error| {
                            self.error(
                                resource_error.clone(),
                                self.bracket(b.v("e"), b.v("r")),
                                error,
                            )
                        };
                        mutant = Some((
                            self.ret(
                                b.v("a"),
                                result.clone(),
                                err(self.public("ResourceError.InvalidBounds")),
                            ),
                            "Ret Err allocate_error replaced by InvalidBounds",
                        ));
                        self.ret(b.v("a"), result.clone(), err(b.v("allocate_error")))
                    } else {
                        let handle = |length| {
                            self.app(
                                self.ctor(self.helpers.mapping_handle),
                                [b.v("resource"), self.call("MkMappingExtent", [length])],
                            )
                        };
                        let release = self.app(
                            self.private(self.helpers.release_if_live),
                            [
                                b.v("a"),
                                resource_error.clone(),
                                b.v("e"),
                                b.v("r"),
                                mapping,
                                b.v("resource"),
                            ],
                        );
                        let bind = |length| {
                            self.bind(
                                b.v("a"),
                                self.body_result(b.v("e"), b.v("r")),
                                result.clone(),
                                self.app(b.v("body"), [handle(length)]),
                                release.clone(),
                            )
                        };
                        mutant = Some((
                            bind(zero.clone()),
                            "Bind body PrivateMappingHandle extent replaced by zero",
                        ));
                        bind(b.v("length"))
                    };
                    (resource_error, left, right)
                }
            }
            "resource_release_settles" => {
                // This row's telescope differs from the acquire rows: the
                // resource kind is an index selected by the caller.
                let mut custom = Binders::default();
                custom.add("a", self.public("Auth"));
                custom.add("o", Term::Type(ken_kernel::Level::zero()));
                custom.add("e", Term::Type(ken_kernel::Level::zero()));
                custom.add("r", Term::Type(ken_kernel::Level::zero()));
                custom.add("kind", self.public("ResourceKind"));
                custom.add("resource", self.resource(custom.v("kind")));
                custom.add(
                    "body_result",
                    self.body_result(custom.v("e"), custom.v("r")),
                );
                let result = self.result(custom.v("o"), self.bracket(custom.v("e"), custom.v("r")));
                let left = self.app(
                    self.private(self.helpers.release_if_live),
                    [
                        custom.v("a"),
                        custom.v("o"),
                        custom.v("e"),
                        custom.v("r"),
                        custom.v("kind"),
                        custom.v("resource"),
                        custom.v("body_result"),
                    ],
                );
                let op = |id| {
                    self.inl(
                        custom.v("a"),
                        self.app(
                            self.ctor(id),
                            [custom.v("a"), custom.v("kind"), custom.v("resource")],
                        ),
                    )
                };
                // Write the actual settlement continuation independently: a
                // release response is settled by the checked generic helper.
                // No private continuation id or law RHS is recovered here.
                let response = self.result(self.public("ResourceError"), self.public("Unit"));
                let mut continuation = custom.clone();
                continuation.add("settled", response.clone());
                let settled = self.app(
                    self.private(self.helpers.settle_result),
                    [
                        continuation.v("o"),
                        continuation.v("e"),
                        continuation.v("r"),
                        continuation.v("body_result"),
                        continuation.v("settled"),
                    ],
                );
                let next = Term::lam(
                    response,
                    self.ret(
                        continuation.v("a"),
                        self.result(
                            continuation.v("o"),
                            self.bracket(continuation.v("e"), continuation.v("r")),
                        ),
                        settled,
                    ),
                );
                let right = self.vis(
                    custom.v("a"),
                    result.clone(),
                    op(self.env.prelude_env.private_bracket_release_id),
                    next.clone(),
                );
                let changed = self.vis(
                    custom.v("a"),
                    result.clone(),
                    op(self.env.prelude_env.private_resource_release_id),
                    next,
                );
                let carrier = self.io(custom.v("a"), result);
                let expected = custom.0.clone().into_iter().rev().fold(
                    self.equal(carrier.clone(), left.clone(), right),
                    |body, (_, domain)| Term::pi(domain, body),
                );
                return (
                    expected,
                    custom.finish(self.equal(carrier, left, changed)),
                    "Vis PrivateBracketRelease replaced by PrivateResourceRelease",
                );
            }
            _ => panic!("no independently authored kernel law `{name}`"),
        };
        let carrier = self.io(
            b.v("a"),
            self.result(error, self.bracket(b.v("e"), b.v("r"))),
        );
        let (changed, perturbation) = mutant.expect("every law has its own perturbation");
        (
            b.0.clone().into_iter().rev().fold(
                self.equal(carrier.clone(), lhs.clone(), rhs),
                |body, (_, domain)| Term::pi(domain, body),
            ),
            b.finish(self.equal(carrier, lhs, changed)),
            perturbation,
        )
    }
}

pub fn expected(env: &ElabEnv, helpers: Helpers, name: &str) -> (Term, Term, &'static str) {
    Kit { env, helpers }.expected(name)
}
