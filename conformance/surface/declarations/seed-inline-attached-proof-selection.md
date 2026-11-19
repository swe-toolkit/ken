# Inline attached-proof selection — conformance seed cases

Format: `../../README.md`. These cases pin `33 §8.2` lookup in inline
children: selection by the resolved subject's checked identity, lookup of
child-owned and provider-exported proofs, and the enclosing-proof position
boundary. `33 §3.1` and `§3.3` ground nested modules and subject resolution.

These are black-box source cases. They assert surface acceptance or refusal,
not internal `GlobalId` values, resolver helpers, or diagnostic spellings.
They do not claim Ken checks, official conformance execution, CI, publication,
landing, or trust measurements.

## surface/declarations/inline-child-selects-own-provider-and-enclosing-proofs

- promise class: durable invariant.
- spec: `32 §1`, `33 §3.1`, `33 §3.2`, `33 §3.3`, `33 §4.1`,
  `33 §8.2`.
- given: in one fresh Strict-root input, define a provider and an outer
  subject, then use all three proof homes from an inline child:

  ```ken
  module Provider {
    pub fn provided_id (x : Int) : Int = x;
    pub proof provided_self for provided_id (x : Int)
      : Equal Int (provided_id x) x = Refl;
  }
  module Outer {
    fn outer_id (x : Int) : Int = x;
    proof outer_self for outer_id (x : Int)
      : Equal Int (outer_id x) x = Refl;
    module Child {
      import Provider (provided_id);
      fn local_id (x : Int) : Int = x;
      proof local_self for local_id (x : Int)
        : Equal Int (local_id x) x = Refl;
      theorem own_proof (x : Int)
        : Equal Int (local_id x) x =
          proof local_self for local_id x;
      theorem provider_proof (x : Int)
        : Equal Int (provided_id x) x =
          provided_id::provided_self x;
      theorem enclosing_proof (x : Int)
        : Equal Int (outer_id x) x =
          proof outer_self for outer_id x;
    };
  }
  ```
- expect: accepts. Each theorem body resolves to the attached proof named
  for its subject: the child's own `local_id` proof, the provider's exported
  proof through the imported `provided_id`, and the enclosing `outer_id`
  proof checked before `Child`.
- measured: when run, the source's surface verdict is acceptance after all
  three theorem bodies are checked.
- claimed: inline-child lookup reaches child-owned, provider-exported, and
  preceding enclosing proofs through their subjects.
- the gap: the three theorem bodies use distinct subject/proof pairs, so a
  missing lookup source cannot be supplied by another proof in this input.
- why: the provider import exposes only the subject as an ordinary name; its
  attached proof remains selected through that subject. No internal proof ID
  is asserted.

## surface/declarations/inline-descendant-shadow-does-not-select-parent-proof

- promise class: durable invariant.
- spec: `32 §1`, `33 §3.1`, `33 §3.3`, `33 §8.2`.
- given: compare two fresh Strict-root inputs. The positive input is:

  ```ken
  module Outer {
    fn same_shape (x : Int) : Int = x;
    proof same_law for same_shape (x : Int)
      : Equal Int (same_shape x) x = Refl;
    module Middle {
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (same_shape x) x =
            proof same_law for same_shape x;
      };
    };
  }
  ```

  The shadowed input adds only a same-spelled, same-typed local subject
  in `Middle` before `Reader`:

  ```ken
  module Outer {
    fn same_shape (x : Int) : Int = x;
    proof same_law for same_shape (x : Int)
      : Equal Int (same_shape x) x = Refl;
    module Middle {
      fn same_shape (x : Int) : Int = x;
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (same_shape x) x =
            proof same_law for same_shape x;
      };
    };
  }
  ```
- expect: the positive input accepts. The shadowed input refuses at surface
  resolution because `Reader` resolves `same_shape` to the distinct
  `Middle.same_shape` subject, which has no `same_law` proof. No particular
  diagnostic kind, message, or span is pinned.
- measured: the unshadowed input accepts and the shadowed input refuses.
- claimed: a `proof_ref` follows the resolved subject's checked identity,
  not just the written spelling.
- the gap: the outer and middle functions have the same type and behavior,
  so selecting the outer proof by spelling would still type-check the
  shadowed theorem. The positive arm proves the selector path is live.
- why: this is a nested local shadow, not an imported-identity or
  middle-import shape. It asserts surface verdicts, not internal identities
  or resolver implementation details.

## surface/declarations/inline-descendant-canonical-shadow-refuses-parent-proof

- promise class: durable invariant.
- spec: `32 §1`, `33 §3.1`, `33 §3.3`, `33 §8.2`.
- given: compare two fresh Strict-root inputs. The positive input is:

  ```ken
  module Outer {
    fn same_shape (x : Int) : Int = x;
    proof same_law for same_shape (x : Int)
      : Equal Int (same_shape x) x = Refl;
    module Middle {
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (same_shape x) x = same_shape::same_law x;
      };
    };
  }
  ```

  The shadowed input adds only a same-spelled, same-typed local subject
  in `Middle` before `Reader` and keeps the canonical path reference:

  ```ken
  module Outer {
    fn same_shape (x : Int) : Int = x;
    proof same_law for same_shape (x : Int)
      : Equal Int (same_shape x) x = Refl;
    module Middle {
      fn same_shape (x : Int) : Int = x;
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (same_shape x) x = same_shape::same_law x;
      };
    };
  }
  ```
- expect: the positive input accepts. The shadowed input refuses at surface
  resolution because `Reader` resolves `same_shape` to the distinct
  `Middle.same_shape` subject, which has no `same_law` proof. No diagnostic
  kind, message, or span is pinned.
- measured: the unshadowed canonical path accepts and the shadowed path
  refuses.
- claimed: the canonical attached path selects by the resolved subject's
  checked identity.
- the gap: both subjects have the same type and behavior, so a spelling-keyed
  lookup of the outer proof would still type-check the shadowed theorem.
  The positive arm proves the canonical path is live.
- why: the only semantic change is the added local shadow. This is a nested
  local subject case, not the D0's imported-subject interaction shape.

## surface/declarations/inline-descendant-enclosing-proof-position-boundary

- promise class: durable invariant.
- spec: `32 §1`, `33 §3.1`, `33 §3.3`, `33 §8.2`.
- given: compare two fresh Strict-root inputs. The accepted control declares
  the enclosing proof before the nested child:

  ```ken
  module Position {
    fn position_id (x : Int) : Int = x;
    proof position_self for position_id (x : Int)
      : Equal Int (position_id x) x = Refl;
    module Shelf {
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (position_id x) x =
            proof position_self for position_id x;
      };
    };
  }
  ```

  The refusal arm differs only by placing the same proof declaration after
  `Shelf`:

  ```ken
  module Position {
    fn position_id (x : Int) : Int = x;
    module Shelf {
      module Reader {
        theorem uses_proof (x : Int)
          : Equal Int (position_id x) x =
            proof position_self for position_id x;
      };
    };
    proof position_self for position_id (x : Int)
      : Equal Int (position_id x) x = Refl;
  }
  ```
- expect: the control accepts. The refusal arm rejects at surface resolution:
  the enclosing proof is declared after the nested child's position, so it
  is not available to that child. Do not pin a diagnostic variant or text.
- measured: the child after the proof accepts; the otherwise identical child
  before it refuses.
- claimed: an inline descendant can select an enclosing proof only when it
  was checked before that child's position.
- the gap: both inputs use the same nested path, subject, proof and selector;
  only the proof's position relative to `Shelf` changes.
- why: a scope-wide lookup that exposes every later enclosing proof would
  accept the refusal arm and fail this pair.

## Clean-room provenance

These cases were derived from the approved §8.2 Spec child, §§3.1 and 3.3,
and first principles. No permissive or copyleft reference was consulted.
