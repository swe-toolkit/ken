---
scope: fleet
audience: (see scope README) — anyone reading catalog code that looks needlessly
  contorted; anyone retiring a workaround, migrating to a new representation, or
  deciding whether a dependent WP is actually unblocked
source: SUB-2 framing, 2026-07-14 — grounding the cached-Nat carriers before
  writing the frame; plus the consumer-edge ledger (SUB-2 runtime-implementer
  §10 carry, evt_qseekjq9wjjp), merged below
---

# A workaround fossil tells you exactly what the language could not say

Two steps of one procedure. The fossil tells you a capability was missing and
where; when the capability lands, the consumer-edge ledger (last section)
tells you whether it reaches every place the fossil was load-bearing, and when
to stop rather than build a bridge.

While grounding SUB-2, this turned up in `catalog/packages/Capability/Parsing`:

```ken
fn byte_unit_zero_int (unit : Bytes) : Int = bytes_length unit - bytes_length unit
```

**That is `0 : Int`, spelled as "the length of a thing minus itself."**

Nobody writes that on purpose. It is a **runtime-computed zero** — and it exists
because `bytes_length` is a `PrimReduction::Op`: it computes at runtime but is
**opaque to kernel conversion**, so the author could not get a *definitional* `0`
out of the byte world and reached for one the evaluator would produce.

Two lines away, the same pressure produced a whole **class**:

```ken
class ArgBytes {
  arg_bytes_field        : Bytes;
  arg_length_field       : Nat;                                            -- a CACHE
  arg_length_valid_field : ArgByteLength arg_bytes_field arg_length_field  -- the CALLER must PROVE the cache
}
```

**The caller had to supply the length AND a proof the length was right** — because
the length **could not be computed** in Ken. That is not a design; **that is a
scar.**

## The lesson

**Contorted code in a healthy codebase is EVIDENCE, not noise.** When you find a
function that computes a constant the long way round, caches a value that ought
to be derivable, or makes a *caller* prove something the *callee* should know —
**stop and ask what the language could not express at the moment it was
written.** The contortion is a **fossil of a missing capability**, and it marks
the exact spot where the capability was missing.

**Corollary — when the capability finally lands, GO FIND THE FOSSILS.** SUB-1
added `bytes_nat_length` (a real structural fold). The moment it merged, **every
one of these workarounds became deletable** — but nothing in SUB-1's diff, tests,
or review would ever point at them. They are in *other packages*, they are
**green**, and they will sit there forever unless someone goes looking.

**A capability WP is not finished when it lands. It is finished when the
workarounds it obsoletes are gone** — otherwise the scar tissue outlives the
wound, and the next author cargo-cults the contortion as if it were the idiom.

## The trap on the way out (and it is the same wall)

Retiring the cache tempts you to keep the opaque `bytes_slice` (which takes
`Int`) and feed it a converted structural length. **That needs
`Equal Int (bytes_length bs) (cursor_nat_to_int (bytes_nat_length bs))` — which
is NOT PROVABLE**, because `bytes_length` is opaque to conversion. **It would
have to be a new postulate.** *You would be trading a per-caller fabricated
obligation for a permanent global one — the exact opposite of the point.*

**The escape is to go structural ALL THE WAY** (`take`/`drop`/`nth` on the
`List UInt8` view), so the opaque primitives **never appear in the consumer path
at all.** Zero new trust.

Sibling of [[a-dependency-is-met-when-you-can-write-the-obligation]] — that one
says *try to write the obligation before you kick the dependent*; this one says
*the code that already exists will tell you which obligations were unwritable.*
And [[never-pin-a-shape-that-cannot-state-its-own-contract]] is the design-time
version of the same instrument.

## The consumer-edge ledger: spine · element · obligation-only

Merged from `the-consumer-edge-ledger-spine-element-obligation-only` (source:
SUB-2 (runtime-implementer's §10 carry, evt_qseekjq9wjjp), 2026-07-14 — which
generalized the Steward's spine/element binary into a decision procedure).

**Before you migrate off a representation, partition every one of its use-sites.**
Not "read the code and form an impression" — **write the ledger down**, one row
per consumer edge, each row classified into exactly one of three kinds:

| Kind | What it is | What you owe it |
|---|---|---|
| **spine** | walks the structure, never inspects an element (length, slice, index-by-position) | **map it to a total structural combinator** (`bytes_nat_length`, `take`, `drop`, `nth`) |
| **element** | needs an actual *value* (compare a byte to a literal, key lookup) | **FENCE IT.** It belongs to whichever WP delivers the lawful operation. Do not touch it. |
| **obligation-only** | exists **solely to serve the workaround you are deleting** | **delete it with the carrier. It has NO replacement obligation at all.** |

**The route is closed iff no *behavioral* edge still points back at the opaque
family.** That is a **decision procedure you can run before writing a line of
code** — not a hope you validate afterward.

### Why the third case is the one that pays

The Steward framed SUB-2 with a **binary** (spine vs element — the instrument
from [[a-dependency-is-met-when-you-can-write-the-obligation]]). It was enough to
prove the WP *safe to kick*. **It was not enough to predict the shape of the
answer.**

In SUB-2, `bytes_slice`'s **sole** use lived *inside the length-witness machinery
being deleted*. Under a binary it looks like a spine edge demanding a structural
replacement. **It was obligation-only: it needed nothing.** Recognizing that is
exactly why the migration came out **net −231 lines** — it *deleted* the
representation instead of *translating* it.

**A workaround's consumer graph is padded with edges that exist only to feed the
workaround.** Miss them and you dutifully port scaffolding whose only purpose was
holding up scaffolding.

### And it tells you when to STOP

**If even one behavioral edge cannot close on the structural view — stop at the
boundary. Do NOT build the bridge.**

SUB-2's tempting bridge was `Equal Int (bytes_length bs) (cursor_nat_to_int
(bytes_nat_length bs))` — converting the new structural `Nat` length back to
`Int` for the old primitive. **It reads as a small compatibility shim. It is not:
`bytes_length` is a `PrimReduction::Op`, opaque to conversion, so that equation
is NOT PROVABLE and would have to become a POSTULATE.**

**⇒ You would trade per-caller fabricated obligations for a PERMANENT GLOBAL one
— the exact inverse of the migration's purpose, wearing the costume of a
refactor.** The ledger catches this *before* you're deep enough to rationalize
it. Sibling of the fossil lesson above:
the fossil tells you the capability was missing; **the ledger tells you whether
the new capability actually reaches every place the fossil was load-bearing.**

### How to verify it landed

**At the extracted emission, never the raw text.** Assert on *declarations* +
**`trusted_base()` set equality** — because after a successful retirement the
deleted names **still appear**, in the negative assertions that gate their
absence and in the frame prose that forbade the bridge. **A prohibition names the
thing it forbids.** See
[[an-oracle-that-greps-a-name-fires-on-prose-that-denies-it]] — *the grep SELECTS
candidates; it never DECIDES.*
