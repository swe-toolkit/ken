# Host effect row labels do not name colliding user families

Format: `../../README.md`.
Spec: `spec/30-surface/36-effects.md` §2.5 and §4.1.
Frame: `docs/program/issues/LANG-PROC-NON-TREE-RESULT-TREE-FS-ROW-AND-ARROW.md`.
Ruling: Architect `evt_4qp0gtvzdmjy0`.

Status: authored; not executed. The accepted `data FsOpenOp` declaration and
tree show that the user family remains valid outside a row. The refused proc
uses `[FsOpen]`, which the ruling reserves for the host effect. A non-host
user-family row with the same declared-tree shape is the accepted control.

## Accepted client

The row label derived from `UserProbeOp` is `UserProbe`, which is not a host
effect label. Its `Vis` tree and proc therefore provide the same-form
non-collision control. The colliding `FsOpenOp` declaration and its user tree
also elaborate here, without naming `FsOpen` in a row.

```ken
data FsOpenOp : Type where {
  UserFsOpenRequest : FsOpenOp
}

fn user_fs_open_resp (op : FsOpenOp) : Type = Unit

const user_fs_open_value : FsOpenOp = UserFsOpenRequest

const user_fs_open_tree : ITree FsOpenOp user_fs_open_resp Unit =
  ITree.Vis FsOpenOp user_fs_open_resp Unit user_fs_open_value
    (λresponse. ITree.Ret FsOpenOp user_fs_open_resp Unit MkUnit)

data UserProbeOp : Type where {
  UserProbeRequest : UserProbeOp
}

fn user_probe_resp (op : UserProbeOp) : Type = Unit

const user_probe_value : UserProbeOp = UserProbeRequest

const user_probe_tree : ITree UserProbeOp user_probe_resp Unit =
  ITree.Vis UserProbeOp user_probe_resp Unit user_probe_value
    (λresponse. ITree.Ret UserProbeOp user_probe_resp Unit MkUnit)

proc user_probe_proc : ITree UserProbeOp user_probe_resp Unit
    visits [UserProbe] =
  user_probe_tree
```

### surface/effects/colliding-user-family-remains-data

- spec: `spec/30-surface/36-effects.md` §2.5.
- promise class: durable invariant.
- given: the accepted client declares `FsOpenOp`, constructs and uses an
  ordinary value of that type, and builds a tree over the family without
  using `FsOpen` in a row.
- expect: the data family, ordinary value, and tree are accepted as
  source declarations.
- claimed: a colliding derived label does not make `data FsOpenOp`
  invalid outside a row.
- gap: this does not establish every possible non-effect use of the
  family.

### surface/effects/user-family-row-label-noncollision

- spec: `spec/30-surface/36-effects.md` §2.5.
- promise class: durable invariant.
- given: `UserProbeOp`, whose derived label `UserProbe` is not a host
  effect label, and a tree-typed proc with `visits [UserProbe]`.
- expect: `user_probe_proc` accepts with its matching user-family tree.
- claimed: a user family can be named by its derived row label when
  that label does not collide with a host effect label.
- gap: one user family does not establish all non-host labels.

## Isolated refusal

Append only the proc below to a fresh copy of the entire accepted client.
The prefix already accepts the `FsOpenOp` family and its tree, as well as
the same-form `UserProbe` proc. The refusal must occur at this procedure's
`visits [FsOpen]` row: `FsOpen` names the host operation, not the user
family whose derived label has the same spelling.

### surface/effects/host-operation-row-label-reserves-colliding-family

- spec: `spec/30-surface/36-effects.md` §2.5; §4.1.
- promise class: durable invariant.
- given: the accepted client and a declared-tree proc returning
  `ITree FsOpenOp user_fs_open_resp Unit`, with `visits [FsOpen]`.
- expect: accept the prefix, then refuse the proc because the host
  `FsOpen` operation has no summand in the user `FsOpenOp` tree. Do not
  require a collision-specific diagnostic string.
- control: `user_probe_proc` has the same result-tree and row shape and
  accepts for its non-host user-family label.
- claimed: a host effect label always denotes that host effect; a user
  family with the same derived label cannot be named in a row.
- gap: this pins the `FsOpen` host-operation spelling, not every host
  family and operation label or cross-file label resolution.

```ken
proc user_fs_open_proc : ITree FsOpenOp user_fs_open_resp Unit
    visits [FsOpen] =
  user_fs_open_tree
```

## Evidence boundary

Architect's ruling `evt_4qp0gtvzdmjy0` makes the `FsOpenOp`/`[FsOpen]`
refusal normative: the row label resolves to host `FsOpen`, and a user
family with that derived label cannot be named in the row. The earlier
observed refusal alone did not establish that rule. The existing
`seed-space-effect-label-reservation.md` covers space declarations, not
written-row resolution; `seed-declared-tree-result-denotation.md` covers
host-operation injection but not this collision. These are corpus scope
notes, not claims that those cases were executed by CV. This seed is
authored, not executed; CV has run no semantic test or official
conformance runner. No CI, publication, or landing is claimed. No external
references or `local/refs/` were consulted.
