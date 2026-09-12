# Inductive-family admission

> **Availability:** partial. **Authority:** explanatory.

`declare_inductive` in the [checking
module](../../../../crates/ken-kernel/src/check.rs) creates a candidate family,
builds its former and constructor types, and provisionally adds it to the global
environment. It then calls the validation path; a validation error removes the
candidate before returning the refusal.

The [positivity checker](../../../../crates/ken-kernel/src/inductive.rs) is one
of those admission mechanisms. `check_positivity` rejects an occurrence of the
family in a forbidden parameter, index, constructor argument, or target-index
position. It validates an inductive declaration; it does not execute an
eliminator.

Eliminator computation is later work. The conversion reducer calls
`iota_reduct` only for a constructor-headed scrutinee belonging to the stated
family. `iota_reduct` refuses mismatched parameter, method, level-argument, or
constructor-spine arities; `whnf` then retains a stuck eliminator when no iota
reduct is available.

The normative family, positivity, and eliminator rules are in the [inductive
specification](../../../../spec/10-kernel/14-inductive.md). Return to [the
kernel landing](../kernel.md) for the surrounding trust boundary.
