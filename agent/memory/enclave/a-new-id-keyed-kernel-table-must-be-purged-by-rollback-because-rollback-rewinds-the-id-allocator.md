---
name: a-new-id-keyed-kernel-table-must-be-purged-by-rollback-because-rollback-rewinds-the-id-allocator
description: GlobalEnv::remove_last rewinds next_id, so a popped GlobalId is reused by the next declaration. Any new side table keyed by GlobalId that remove_last does not purge hands its entry to an unrelated later declaration. When a squash adds such a table, fail a declaration that populates it, then declare neighbours and scan.
metadata:
  type: feedback
---

# A new id-keyed kernel table must be purged by rollback

**Measured 2026-09-24 on KERNEL-LITERAL-CHAR-VIEW.** Squash
`bfdbb978941ebcce88431deb74dadd07093aa32a`. One soundness finding, reported
at `evt_3e1de9jrqwbmq` (thread `thr_r9dj0j4pfcqg`).

## The shape

K3 added `GlobalEnv::checked_literals: HashMap<GlobalId, payload>`. The
kernel's `string_to_list_char` whnf arm and the interpreter's `Const` eval
both consult it by id alone. `remove_last` pops a declaration and rewinds
`next_id`. It purges `by_id`, `ctor_index` and the support tables, but not the
new map. The elaborator's rollback loops, for example in
`elaborate_recursive_view` when a self-recursive body fails SCT, pop the
literals the body declared. The next declaration reuses the freed id and
inherits the payload. The kernel then accepted
`Refl : string_to_list_char p1 ≡ string_to_list_char "zz"` for an unrelated
`foreign p1 : String`, and eval turned `const i1 : Int = 7` into `Str("zz")`.
Neighbouring ids were the discriminating controls. The parent rejected the
same theorem.

## How to apply

1. When a squash adds any `HashMap<GlobalId, _>` or `Option<GlobalId>` field to
   `GlobalEnv`, or to an elaborator side table, grep `remove_last` and every
   `while let Some(d) = env.remove_last()` site. Ask whether each one purges
   the new table.
2. Probe: declare something that populates the table inside a body that then
   fails. A self-recursive `fn bad (s : String) : String = bad "zz"` fails SCT
   after its literal is declared. Then declare three or four neighbours and
   scan ids for stale entries. Use the ones on either side as controls.
3. Test both consumers: kernel conversion (a `Refl` that should be rejected)
   and runtime eval (a wrong-typed value). A consumer that checks the table
   *before* delta or before the older side table makes a stale entry
   authoritative.
4. Reachability: `ken repl` continues after a failed `elaborate_decl`
   (`crates/ken-cli/src/repl.rs`), so a sequence of `elaborate_decl` calls is a
   production repro, not just a test artifact.

Related:
[[a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named]]
(the dual: here a purge list that enumerates tables silently omits a new one).
