---
name: write-ken
description: Select the smallest checked Ken product-context pack before writing, proving, reviewing, or diagnosing Ken source.
scope: tools
model: claude-sonnet-5
---

# Write Ken

You are about to write, prove, review, or diagnose Ken source. Before changing
a line, load the product-context selection protocol at
`library/agents/README.md`, then select exactly one pack from
`library/agents/packs/` whose triggers match the task and whose exclusions do
not. If the change has no Ken source lines, this skill does not apply (see
Scope).

## Scope

Ken source is `.ken` and `.ken.md` files and the Ken fences in `spec/`,
`conformance/` and `catalog/`. This skill covers only that.

Rust, C companions, the host effect catalog, tests written in Rust, scripts and
CI are not Ken source. Write or review them under your role playbook. They need
no pack, and the lack of a pack for them is never a stop.

## Select one pack

| Task | Pack |
|---|---|
| explain or review existing source | `read-review` |
| write a pure program and law | `write-pure` |
| write an effectful boundary | `write-effectful` |
| find, use, or author a catalog package | `author-package` |
| repair a proof without adding trust | `repair-proof` |
| diagnose a parse-to-runtime failure | `diagnose` |

Load pack dependencies first and module includes in their listed order. Follow
a module prerequisite only when the task reaches it. Do not load the entire
agent library by default.

## Judge exclusions by the Ken lines

Judge the platform exclusions (`platform or FFI work`, `platform ABI work`)
against the Ken lines you are writing or reviewing, not against the WP's title,
lane or other files. Other exclusions apply as written. The README's "if a task
needs one" means the same: the Ken lines need one.

A Ken file that only calls a host operation through the prelude or catalog is
in scope inside an ABI or runtime WP: `read-review` to review it,
`write-effectful` to write it. That holds when the same WP adds the operation
on the host side; report that dependency in your verdict.

## When to stop

Stop and report the unsupported boundary when the Ken source itself needs what
no pack covers. This wave has no FFI/platform pack, so that means a `foreign`
declaration, or Ken source that names a host operation the host does not
provide. Do not invent a package, primitive, effect, capability, proof,
command, or syntax to bridge a missing module.

The stop covers only the Ken lines that need it. Report them, and continue the
rest of the WP under your role playbook.

You will be tempted to stop because a WP is labelled ABI, platform or runtime.
First check the changed Ken lines for a `foreign` declaration or a call the
host does not provide. If there is none, select the pack and continue.

## After the task

After the product task is complete, follow the role and team playbooks
already active in the session for checks, review, and handoff.

Route findings instead of leaving them in task notes: gaps in authoring
guidance go to the relevant `catalog/guide/` strand; recurring sugar candidates
and confusing manual elaboration go to Ergo; reusable `def`/`theorem`/`prop`
artifacts go into the catalog; kernel-reduction defects go to Kernel through
the enclave.
