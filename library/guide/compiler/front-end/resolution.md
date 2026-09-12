# Resolution

> **Availability:** partial. **Authority:** explanatory.

The [resolver](../../../../crates/ken-elaborator/src/resolve.rs) translates
surface declarations and expressions into resolved forms. Its public
`resolve_decls` operation processes a declaration sequence with a shared set of
unit definitions; the result contains resolved declarations rather than the
parser’s source-only tree.

Resolution accounts for lexical scope and declaration-name collisions. Its
scope records bindings and assigns de Bruijn-style positions to a locally bound
variable. A local scope miss becomes `RCon`, preserving the name for the
elaborator's later global lookup; it is not a general resolver-stage rejection.
`resolve_expr_ctx` has a narrower refusal for `result` outside its permitted
contract context.

Resolved syntax still is not admitted core. The resolver makes names and source
contexts explicit for the [elaborator](elaboration-and-admission.md), which
consults its global environment and can reject an unknown global name before it
constructs core terms and asks the kernel to check them. The source language’s
name and module rules are normative in the [declarations
chapter](../../../../spec/30-surface/33-declarations.md), not in this guide.
