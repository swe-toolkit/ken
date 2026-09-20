# `Tooling.Testing.Property` — deterministic finite property checks

Availability: **partial**

Authority: **derived reference**

Canonical source: [checked package](../../../../../catalog/packages/Tooling/Testing/Property.ken.md).

| Fact class | Disposition | Result and grounding |
|---|---|---|
| Subject | `generated` | `catalog/packages/Tooling/Testing/Property.ken.md` — “`Tooling.Testing.Property` — deterministic finite property checks,” from the [subject index](../../subjects.md). |
| Declaration/type | `authored` | The package declares no public surface. Its private implementation provides deterministic finite generators and checking over the ordinary `Result a Unit` carrier, with concrete executable witnesses. |
| Law | `authored` | Private attached laws for the runner establish successful-lookup soundness, success completeness, first-counterexample recovery, and the two `gen_from_list` coherences. They are checked implementation laws, not public declarations. |
| Effect/capability | `none-declared` | The canonical checked fences declare no effect row, `proc`, `visits`, or capability value; the design explicitly omits effects. |
| Assurance | `authored` | The checked implementation has zero trusted-base delta and introduces no primitive, postulate, axiom, proof hole, effect, or assumed proposition. Its deterministic finite samples make the first counterexample reproducible. |
| Platform | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Maturity | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Dependency | `held` | No complete package-level checked projection exists; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Reverse dependency | `held` | No complete projection and maintained inversion exist; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
