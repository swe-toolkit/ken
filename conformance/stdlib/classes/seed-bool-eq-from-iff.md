# Boolean equality from mutual implication conformance — seed

Format: `../../README.md`.
Spec: target §9 of `spec/50-stdlib/51-lawful-classes.md`, per Architect
AC-0 `evt_49tdm0dmc40pt`; theorem syntax: `spec/30-surface/33-declarations.md`
§8.3.

This seed is authored but unexecuted. No claim is made that the official
conformance runner has executed it.

## Accepted open client

The client applies `bool_eq_from_iff` to open Boolean values using both truth
directions. Its only explicit import is the public theorem.

```ken
import Core.Classes.LawfulClasses (bool_eq_from_iff)

theorem bool_eq_open (p : Bool) (q : Bool)
    (forward : Equal Bool p True → Equal Bool q True)
    (backward : Equal Bool q True → Equal Bool p True)
    : Equal Bool p q = bool_eq_from_iff p q forward backward
```

### stdlib/classes/bool-eq-from-iff-open-client

- promise class: durable invariant.
- spec: `51 §9`, public theorem `bool_eq_from_iff`.
- given: the open client above, with only its explicit `LawfulClasses`
  import.
- expect: `bool_eq_open` is accepted.
- measured: whether the checker accepts this open client with both
  implications.
- claimed: the public theorem is usable with open Boolean values when both
  truth directions are supplied.
- gap: this client checks import and application, not the theorem's module
  ownership or transparency.
- why: the client uses the public result without `trans` or `sym` imports.

## Isolated forward-only refusal

Start from a fresh prefix containing only the same explicit import. Do not
include `bool_eq_open` or any other declaration before the refusal.

```ken
import Core.Classes.LawfulClasses (bool_eq_from_iff)

theorem bool_eq_forward_only (p : Bool) (q : Bool)
    (forward : Equal Bool p True → Equal Bool q True)
    : Equal Bool p q = bool_eq_from_iff p q forward
```

### stdlib/classes/bool-eq-from-iff-requires-both-directions

- promise class: durable invariant.
- spec: `51 §9`, public theorem `bool_eq_from_iff`.
- given: the fresh prefix above and only the forward-only theorem.
- expect: the declaration is rejected within its own theorem with
  `KernelRejected(TypeMismatch)`. Do not pin a span or diagnostic wording.
- measured: whether the checker rejects this declaration with the expected
  type mismatch.
- claimed: the forward truth implication alone cannot establish
  `Equal Bool p q`.
- gap: the refusal is attributable to the missing direction only if the
  import prefix resolves; the open-client case is that positive control.
- why: `p = False` and `q = True` satisfy the forward implication
  vacuously but are unequal; the reverse implication is load-bearing.

The closed `Refl` and `Proved` controls are package-test controls, not seed
cases. No `trans` or `sym` imports are needed. No external references were
consulted; this seed is grounded in the AC-0 ruling and first principles.
