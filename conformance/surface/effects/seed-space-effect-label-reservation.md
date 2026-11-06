# Host effect labels are reserved at space declarations

This seed pins the reserved-label rule in `36 §4.1`, added after the bullet
"one effect label for the space." A space name may not be a host effect label:
that includes a host-family label and a host-operation label. Since rows compare
labels by name, accepting a same-named space would charge or discharge its
operations as the host effect. The space declaration is refused instead.

The host-family labels are `Console`, `Clock`, `Entropy`, `FS`, and `Process`.
Host-operation labels come from the host operation inventory; `FsOpen` and
`ResourceRelease` are representative values. The conformance population for
that arm is the inventory's operation-label set, not a second hand-maintained
list of its current members.

This is a surface/elaboration rule. It does not change how a written row label
is resolved, and it does not specify cross-file space-name behavior. In
particular, this seed makes no claim about `visits [Bogus]` or cross-file same-
name spaces.

The name-specific error is `SpaceNameIsEffectLabel` (`E0088`), as settled by the
Architect's AC-0 ruling. No official conformance runner result is claimed.

## surface/effects/space-name-rejects-host-family-label

- spec: `30-surface/36-effects.md §4.1`, §1.1; `33-declarations.md §3.3`
- given: use this one-file space shape as the accepted control:

  ```ken
  space Other {
    mut cell : Int = 0
    proc read () : Int visits [Other] = cell
  }
  ```

  For the refusal arms, use the same declarations and body, changing `Other`
  consistently to one of `Console`, `Clock`, `Entropy`, `FS`, or `Process`.
  The space need not be called by another declaration.
- expect: the `Other` control accepts. Each host-family-name variant refuses
  at the `space` declaration with `SpaceNameIsEffectLabel` (`E0088`), naming
  the reserved label. This holds even when the space is otherwise unused.
- why: the pair varies only whether the introduced space label is already a
  host-family label. It prevents both a spelling-alias acceptance and a blanket
  refusal of ordinary user space names.

## surface/effects/space-name-rejects-host-operation-label

- spec: `30-surface/36-effects.md §4.1`, §1.1
- given: use the accepted `Other` shape above. Instantiate the refusal arm with
  a space named `FsOpen`; also exercise `ResourceRelease`. For the complete
  operation-label population, instantiate the same pair for each host
  operation label in the host inventory.
- expect: the `Other` control accepts. A space named with any host-operation
  label refuses at its declaration with `SpaceNameIsEffectLabel` (`E0088`),
  naming that label. This does not depend on the space performing the operation
  or being called.
- why: a host-operation label is reserved just like a host-family label. The
  inventory-defined population prevents the rule from stopping at one example
  or at the family-label branch.

## surface/effects/space-label-alias-refuses-at-declaration

- spec: `30-surface/36-effects.md §4.1`, §1.1
- given: the measured alias program:

  ```ken
  space Console {
    mut cell : Int = 0
    proc call () : Int visits [Console] = 1
  }
  proc user_console (n : Int) : Int visits [Console] =
    let m : Int = Console.call in m
  ```

  The same-form `Other` control declares `space Other`, its operation uses
  `visits [Other]`, and `user_console` calls `Other.call` under
  `visits [Console]`.
- expect: the `Console` program refuses at the `space Console` declaration
  with `SpaceNameIsEffectLabel` (`E0088`), before the later row/call behavior.
  The `Other` control reaches the existing `EffectEscapes` error (`E0038`)
  for `Other` in `user_console`; it does not report a reserved-label error.
- why: this preserves the original spelling-collision discriminator while
  requiring the new rule to refuse at its specified introduction point. The
  control distinguishes that rule from the ordinary row escape.

## surface/effects/space-operation-alias-refuses-at-declaration

- spec: `30-surface/36-effects.md §4.1`, §1.1
- given: a space operation that performs ambient Console output:

  ```ken
  space Console {
    mut cell : Int = 0
    proc call () : Unit visits [Console] = print_line "x"
  }
  ```

  The same-form control names the space `Other` and gives its operation
  `visits [Other]`.
- expect: the `Console` program refuses at the space declaration with
  `SpaceNameIsEffectLabel` (`E0088`), rather than failing later while
  elaborating the operation. The `Other` control retains the existing
  `EffectEscapes` (`E0038`) result for ambient `Console` in `Other.call`.
- why: the declaration gate wins before an operation body can make the
  same-spelled row label look like an ordinary space effect. The control
  keeps that property distinct from the existing row check.

Ordinary ambient Console use without a same-named user space remains covered
by `seed-effects.md`; this seed does not duplicate the row/operation contract.

## Boundary of this seed

The row-error control intentionally keeps `visits [Bogus]` out of scope. The
rule under test concerns names introduced by a local `space`, not unresolved
written-row labels. Cross-file same-name spaces also remain outside this seed's
contract and coverage.
