# Explicit core terms

> **Availability:** partial. **Authority:** explanatory.

The [core `Term` type](../../../../crates/ken-kernel/src/term.rs) is the
kernel's input language. Its variants represent universes, de Bruijn variables,
global declarations, dependent functions and pairs, applications, eliminators,
and the other explicit forms the checker handles. The term representation is
not surface syntax: the elaborator produces it before the kernel is asked to
admit a declaration.

Binder-bearing variants carry the binder convention in the data itself. In
`Pi`, `Lam`, `Sigma`, and `Let`, the bound variable has index zero in the body;
the [substitution module](../../../../crates/ken-kernel/src/subst.rs) supplies
the corresponding structural operations. A malformed de Bruijn reference is
not repaired by this representation: later raw well-formedness and inference
consult a context and can refuse an out-of-scope index.

The core syntax is normative in the [kernel syntax
specification](../../../../spec/10-kernel/11-syntax.md). This page identifies
the Rust representation that carries it. Next read [contexts and
declarations](contexts-and-declarations.md), which supplies the environments
against which a term is checked.
