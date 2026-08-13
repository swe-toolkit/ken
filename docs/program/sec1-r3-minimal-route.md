# SEC1 R3 minimal-route report

This report measures what the specified `product(c, ζ)` obligation would need
before a deliberately weak postcondition can yield a kernel-accepted
`q : φ → Bottom`. It does not discharge an `AC-R3` row, implement a prover
component, or recommend changing the solver policy.

## Measurement one: the obligation routes to FO

The Sec1 reduction emits
`lowEq_ζ(in¹, in²) ⇒ (lowEq_ζ(out¹, out²) ∧
coterminates_ζ(c¹, c²))` (`spec/60-security/61-information-flow.md` §5.3,
lines 342–349). Treating the three named predicates as applications, as that
specification presents them, gives a `Pi` over a `Sigma` of applications.
Those constructors are all accepted by `is_first_order_intuit`
(`crates/ken-elaborator/src/prover.rs:172-185`), so `classify` selects `FO`
(`prover.rs:139-150`). It does not select the ground-decidable `D` route whose
four deferred components are named at `prover.rs:317-318`.

A disposable targeted classifier probe constructed exactly that
`Pi(App, Sigma(App, App))` shape and observed `Route::FO`. Its positive
discriminator replaced one postcondition atom with a direct kernel `Eq`, which
`is_first_order_intuit` explicitly excludes, and observed `Route::HO`. The
probe passed and was removed; no probe or crate change is part of this report.

The four requested component dispositions are therefore:

| component | disposition | reason |
|---|---|---|
| kernel whnf | not required as a new prover component | A successful verdict must still end in the existing kernel check (`spec/20-verification/23-prover.md` §1.5), which may normalize while checking. But normalization alone neither discovers nor constructs `q`, and the D placeholder's proposed whnf step is not reached by this FO shape. |
| decision procedure (`23` §3.1) | not required | Reflective decision is specified for concrete or closed decidable goals (`23-prover.md` §3, lines 145–158). This obligation has the first-order variables introduced by the two-run product. |
| solver-backed arithmetic search | required only in combination with the Kripke embedding and a checked certificate path under the specified automated FO architecture | FO obligations cannot be sent directly to a classical solver (`23-prover.md` §4, lines 166–200). The specified route is embedding, solver search, then either the verified checker and adequacy theorem or reconstruction (`23` §4, lines 198–218). Solver output alone cannot produce an accepted `q`. |
| `Decidable` constructor extraction (`23` §3.2) | not required | Constructor extraction belongs to the D reflective-decision route (`23-prover.md` §3). The FO trust step instead names embedding adequacy plus a verified certificate checker, or reconstruction (`23` §4). |

None of the four D-route components is sufficient alone. The binding missing
piece in the specified route is the FO Kripke embedding and certificate path,
which `attempt_fo` itself records as a placeholder
(`prover.rs:326-341`). Solver search participates only with that path. It is not
a substitute for it and is not part of the trusted path.

This is a statement about the currently specified automated architecture, not
a theorem that the Ken language forbids every other proof. A manually supplied
kernel proof or a future native constructive tactic could avoid external solver
search. No such production route was found or tried here. The existing solver
deferral remains the operator's ruling
(`docs/program/03-program-of-work.md:182-196`); this report does not revisit or
weigh it.

## Measurement two: widening is not a caller-count question

There are two distinct registries:

- `NumericEnv::eq_table` maps a surface operand type to an equality operation;
  it is populated for `Int`, `Float`, and `Float32`
  (`crates/ken-elaborator/src/numbers.rs:140,180-205,551-554`) and separately
  for `Char` (`decimal_char.rs:262-264`).
- `GlobalEnv::deceq_certs` is the kernel proof-equality gate. `eq_reduce`
  consults it only for a primitive constant with a registered certificate;
  an unregistered primitive remains neutral
  (`crates/ken-kernel/src/obs.rs:75-94`).

No production bridge from an `EqEntry` to a `DecEqCert` was found. Such a
bridge would not be a lookup adapter: an `EqEntry` contains only an operation
id, while `declare_deceq_certificate` constructs and registers separate
universal soundness and completeness laws
(`crates/ken-kernel/src/check.rs:1293-1321`). Automatically treating every
surface equality operation as proof equality would therefore be a design
change and each distinct primitive registration would be a registration.

The three required dispositions are:

1. `Char` has a second surface `EqEntry`, but it does not require a second
   kernel certificate. ADR 0013 records that `Char` is a refinement whose
   carrier lowers to `Int`, so `Eq Char` bottoms out at `Eq Int`
   (`docs/adr/0013-int-decidable-equality-kernel-posture.md:10-14,75-80`). A
   separate `Char` primitive registration would duplicate trust rather than
   expose a missing lawful registrant.
2. `eq_float` and `eq_float32` are deliberately not proof equality. The
   primitive registry says they carry no `DecEq`/`Eq` law
   (`spec/10-kernel/18a-primitive-registry.md` §5.4), and the numeric spec names
   NaN, signed zero, and rounding as the IEEE equality boundary
   (`spec/30-surface/35-numbers.md` §2.4). In particular, NaN violates the
   reflexivity expected of propositional equality; IEEE equality also equates
   positive and negative zero even though substituting one for the other can
   change later IEEE results such as division's sign.
   Bridging either operation is semantically forbidden by its present
   non-proof contract, not an omitted registration.
3. A new distinct opaque primitive registrant would grow the trusted base.
   The kernel does not execute `PrimReduction::Op`, so universal laws over
   abstract values cannot compute (ADR 0013, lines 17–25).
   `declare_deceq_certificate` admits one soundness and one completeness
   postulate (`check.rs:1302-1313`) and then records the pair. The resulting
   TCB growth is reported here without weighing whether to accept it.

The earlier statement that widening is vacuous reaches the right current
outcome for the wrong reason. The surface registry does contain other equality
operations, but `Char` reuses `Int` definitionally and the two float operations
are intentionally non-proof. No second lawful, distinct kernel registrant was
found. The language does not forbid registering some future opaque primitive;
under the present mechanism that action would add the two trusted universal
laws above and therefore requires the separate TCB decision.

## Evidence and limits

The targeted observations were:

```text
scripts/ken-cargo test -p ken-elaborator \
  --test sec1_r3_route_probe -- --test-threads=1
1 passed; disposable probe removed

scripts/ken-cargo test -p ken-kernel \
  --test ds6a_int_deceq_certificate -- --test-threads=1
5 passed

scripts/ken-cargo test -p ken-kernel \
  --test ds6b_intlit_eq_reduction -- --test-threads=1
9 passed
```

The registry search covered the production tree and found
`declare_deceq_certificate` registering only `Int`; it found no conversion
from `NumericEnv::EqEntry` to `GlobalEnv::DecEqCert`. This report did not build
any of the four deferred prover components, generate the product program, run
an actual weak-postcondition obligation, add a registry bridge, register a
second type, or test an alternate proof route. Consequently, “not found” for
an alternate proof route and a second lawful registrant is an observed
repository boundary, not a language prohibition. The float disposition is
different: their specified IEEE operations are explicitly non-proof equality.
