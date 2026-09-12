# Contexts and declarations

> **Availability:** partial. **Authority:** explanatory.

The [context and environment
structures](../../../../crates/ken-kernel/src/env.rs) provide the kernel's local
and global lookup state. `Context::push` extends the local telescope, and
`Context::lookup` translates a de Bruijn index from the innermost position. A
lookup outside that telescope returns no type; `infer` is the later consumer
that turns that absence into a `VarOutOfScope` refusal.

`GlobalEnv` records top-level `Decl` values, including transparent definitions,
opaque declarations, inductive families, and primitives. The environment is
append-only and uses declaration identities for lookup. The declaration data
records whether a definition is transparent, but the conversion reducer is the
later mechanism that decides whether and how to unfold it.

These data structures do not themselves admit a declaration. The declaration
admission functions in [checking and admission](checking-and-admission.md)
validate candidate declarations before they become useful kernel state. The
normative context and environment rules are in [kernel syntax
§3–4](../../../../spec/10-kernel/11-syntax.md).
