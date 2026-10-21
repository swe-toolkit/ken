# Finite event traces

Availability: **current**

Authority: **derived reference**

Canonical source: [checked package](../../../../../catalog/packages/Data/Time/EventTrace.ken.md).

| Fact class | Disposition | Result and grounding |
|---|---|---|
| Subject | `generated` | `catalog/packages/Data/Time/EventTrace.ken.md` — “Finite event traces,” from the [subject index](../../subjects.md). |
| Declaration/type | `authored` | Declares finite timestamped events, time and payload projections, ordering and half-open window predicates, window filtering, and a left-biased merge of two event lists. |
| Law | `authored` | `trace_window_ordered` preserves ordering for an ordered input. `trace_merge_ordered` and `trace_window_merge` require both inputs ordered. Window-membership and merge-count laws do not require ordered inputs. |
| Effect/capability | `none-declared` | The canonical checked fences declare no effect row, `proc`, `visits`, or capability value for this package. |
| Assurance | `authored` | The package consists of checked definitions and proofs; its source reports no added trusted-base assumptions. |
| Platform | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Maturity | `held` | No checked per-package facet is available; owner: catalog campaign. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Dependency | `held` | No complete package-level checked projection exists; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
| Reverse dependency | `held` | No complete projection and maintained inversion exist; owner: `crates/`. See the [held-class disclosure](../../card-format.md#held-class-disclosure). |
