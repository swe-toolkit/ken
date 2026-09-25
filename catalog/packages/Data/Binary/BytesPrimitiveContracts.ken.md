# Contracts for primitive byte operations

Four named facts connect the opaque byte primitives to their structural views.
The finite ASCII witness is ordinary checked Ken data, not a fifth fact. It is
shared below Parsing; no primitive or parser definition is changed here.

## 1. Checked ASCII witnesses

`AsciiTag` has exactly 128 constructors. `AsciiCode n` pairs a tag with a
checked equality to its value, so a code outside `0..127` has no witness.
`AllAsciiCodes` carries one such witness for each element of a code list.
K3's checked literal view makes the witness for a concrete ASCII `String`
reducible without an encoder assumption.

```ken
import Data.Collections.Derived (list_append, map)

pub data AsciiTag : Type where {
  Ascii0;
  Ascii1;
  Ascii2;
  Ascii3;
  Ascii4;
  Ascii5;
  Ascii6;
  Ascii7;
  Ascii8;
  Ascii9;
  Ascii10;
  Ascii11;
  Ascii12;
  Ascii13;
  Ascii14;
  Ascii15;
  Ascii16;
  Ascii17;
  Ascii18;
  Ascii19;
  Ascii20;
  Ascii21;
  Ascii22;
  Ascii23;
  Ascii24;
  Ascii25;
  Ascii26;
  Ascii27;
  Ascii28;
  Ascii29;
  Ascii30;
  Ascii31;
  Ascii32;
  Ascii33;
  Ascii34;
  Ascii35;
  Ascii36;
  Ascii37;
  Ascii38;
  Ascii39;
  Ascii40;
  Ascii41;
  Ascii42;
  Ascii43;
  Ascii44;
  Ascii45;
  Ascii46;
  Ascii47;
  Ascii48;
  Ascii49;
  Ascii50;
  Ascii51;
  Ascii52;
  Ascii53;
  Ascii54;
  Ascii55;
  Ascii56;
  Ascii57;
  Ascii58;
  Ascii59;
  Ascii60;
  Ascii61;
  Ascii62;
  Ascii63;
  Ascii64;
  Ascii65;
  Ascii66;
  Ascii67;
  Ascii68;
  Ascii69;
  Ascii70;
  Ascii71;
  Ascii72;
  Ascii73;
  Ascii74;
  Ascii75;
  Ascii76;
  Ascii77;
  Ascii78;
  Ascii79;
  Ascii80;
  Ascii81;
  Ascii82;
  Ascii83;
  Ascii84;
  Ascii85;
  Ascii86;
  Ascii87;
  Ascii88;
  Ascii89;
  Ascii90;
  Ascii91;
  Ascii92;
  Ascii93;
  Ascii94;
  Ascii95;
  Ascii96;
  Ascii97;
  Ascii98;
  Ascii99;
  Ascii100;
  Ascii101;
  Ascii102;
  Ascii103;
  Ascii104;
  Ascii105;
  Ascii106;
  Ascii107;
  Ascii108;
  Ascii109;
  Ascii110;
  Ascii111;
  Ascii112;
  Ascii113;
  Ascii114;
  Ascii115;
  Ascii116;
  Ascii117;
  Ascii118;
  Ascii119;
  Ascii120;
  Ascii121;
  Ascii122;
  Ascii123;
  Ascii124;
  Ascii125;
  Ascii126;
  Ascii127
}

export Ascii0,
  Ascii1,
  Ascii2,
  Ascii3,
  Ascii4,
  Ascii5,
  Ascii6,
  Ascii7,
  Ascii8,
  Ascii9,
  Ascii10,
  Ascii11,
  Ascii12,
  Ascii13,
  Ascii14,
  Ascii15,
  Ascii16,
  Ascii17,
  Ascii18,
  Ascii19,
  Ascii20,
  Ascii21,
  Ascii22,
  Ascii23,
  Ascii24,
  Ascii25,
  Ascii26,
  Ascii27,
  Ascii28,
  Ascii29,
  Ascii30,
  Ascii31,
  Ascii32,
  Ascii33,
  Ascii34,
  Ascii35,
  Ascii36,
  Ascii37,
  Ascii38,
  Ascii39,
  Ascii40,
  Ascii41,
  Ascii42,
  Ascii43,
  Ascii44,
  Ascii45,
  Ascii46,
  Ascii47,
  Ascii48,
  Ascii49,
  Ascii50,
  Ascii51,
  Ascii52,
  Ascii53,
  Ascii54,
  Ascii55,
  Ascii56,
  Ascii57,
  Ascii58,
  Ascii59,
  Ascii60,
  Ascii61,
  Ascii62,
  Ascii63,
  Ascii64,
  Ascii65,
  Ascii66,
  Ascii67,
  Ascii68,
  Ascii69,
  Ascii70,
  Ascii71,
  Ascii72,
  Ascii73,
  Ascii74,
  Ascii75,
  Ascii76,
  Ascii77,
  Ascii78,
  Ascii79,
  Ascii80,
  Ascii81,
  Ascii82,
  Ascii83,
  Ascii84,
  Ascii85,
  Ascii86,
  Ascii87,
  Ascii88,
  Ascii89,
  Ascii90,
  Ascii91,
  Ascii92,
  Ascii93,
  Ascii94,
  Ascii95,
  Ascii96,
  Ascii97,
  Ascii98,
  Ascii99,
  Ascii100,
  Ascii101,
  Ascii102,
  Ascii103,
  Ascii104,
  Ascii105,
  Ascii106,
  Ascii107,
  Ascii108,
  Ascii109,
  Ascii110,
  Ascii111,
  Ascii112,
  Ascii113,
  Ascii114,
  Ascii115,
  Ascii116,
  Ascii117,
  Ascii118,
  Ascii119,
  Ascii120,
  Ascii121,
  Ascii122,
  Ascii123,
  Ascii124,
  Ascii125,
  Ascii126,
  Ascii127

pub fn ascii_value (tag : AsciiTag) : Int =
  match tag {
    Ascii0 ↦ 0;
    Ascii1 ↦ 1;
    Ascii2 ↦ 2;
    Ascii3 ↦ 3;
    Ascii4 ↦ 4;
    Ascii5 ↦ 5;
    Ascii6 ↦ 6;
    Ascii7 ↦ 7;
    Ascii8 ↦ 8;
    Ascii9 ↦ 9;
    Ascii10 ↦ 10;
    Ascii11 ↦ 11;
    Ascii12 ↦ 12;
    Ascii13 ↦ 13;
    Ascii14 ↦ 14;
    Ascii15 ↦ 15;
    Ascii16 ↦ 16;
    Ascii17 ↦ 17;
    Ascii18 ↦ 18;
    Ascii19 ↦ 19;
    Ascii20 ↦ 20;
    Ascii21 ↦ 21;
    Ascii22 ↦ 22;
    Ascii23 ↦ 23;
    Ascii24 ↦ 24;
    Ascii25 ↦ 25;
    Ascii26 ↦ 26;
    Ascii27 ↦ 27;
    Ascii28 ↦ 28;
    Ascii29 ↦ 29;
    Ascii30 ↦ 30;
    Ascii31 ↦ 31;
    Ascii32 ↦ 32;
    Ascii33 ↦ 33;
    Ascii34 ↦ 34;
    Ascii35 ↦ 35;
    Ascii36 ↦ 36;
    Ascii37 ↦ 37;
    Ascii38 ↦ 38;
    Ascii39 ↦ 39;
    Ascii40 ↦ 40;
    Ascii41 ↦ 41;
    Ascii42 ↦ 42;
    Ascii43 ↦ 43;
    Ascii44 ↦ 44;
    Ascii45 ↦ 45;
    Ascii46 ↦ 46;
    Ascii47 ↦ 47;
    Ascii48 ↦ 48;
    Ascii49 ↦ 49;
    Ascii50 ↦ 50;
    Ascii51 ↦ 51;
    Ascii52 ↦ 52;
    Ascii53 ↦ 53;
    Ascii54 ↦ 54;
    Ascii55 ↦ 55;
    Ascii56 ↦ 56;
    Ascii57 ↦ 57;
    Ascii58 ↦ 58;
    Ascii59 ↦ 59;
    Ascii60 ↦ 60;
    Ascii61 ↦ 61;
    Ascii62 ↦ 62;
    Ascii63 ↦ 63;
    Ascii64 ↦ 64;
    Ascii65 ↦ 65;
    Ascii66 ↦ 66;
    Ascii67 ↦ 67;
    Ascii68 ↦ 68;
    Ascii69 ↦ 69;
    Ascii70 ↦ 70;
    Ascii71 ↦ 71;
    Ascii72 ↦ 72;
    Ascii73 ↦ 73;
    Ascii74 ↦ 74;
    Ascii75 ↦ 75;
    Ascii76 ↦ 76;
    Ascii77 ↦ 77;
    Ascii78 ↦ 78;
    Ascii79 ↦ 79;
    Ascii80 ↦ 80;
    Ascii81 ↦ 81;
    Ascii82 ↦ 82;
    Ascii83 ↦ 83;
    Ascii84 ↦ 84;
    Ascii85 ↦ 85;
    Ascii86 ↦ 86;
    Ascii87 ↦ 87;
    Ascii88 ↦ 88;
    Ascii89 ↦ 89;
    Ascii90 ↦ 90;
    Ascii91 ↦ 91;
    Ascii92 ↦ 92;
    Ascii93 ↦ 93;
    Ascii94 ↦ 94;
    Ascii95 ↦ 95;
    Ascii96 ↦ 96;
    Ascii97 ↦ 97;
    Ascii98 ↦ 98;
    Ascii99 ↦ 99;
    Ascii100 ↦ 100;
    Ascii101 ↦ 101;
    Ascii102 ↦ 102;
    Ascii103 ↦ 103;
    Ascii104 ↦ 104;
    Ascii105 ↦ 105;
    Ascii106 ↦ 106;
    Ascii107 ↦ 107;
    Ascii108 ↦ 108;
    Ascii109 ↦ 109;
    Ascii110 ↦ 110;
    Ascii111 ↦ 111;
    Ascii112 ↦ 112;
    Ascii113 ↦ 113;
    Ascii114 ↦ 114;
    Ascii115 ↦ 115;
    Ascii116 ↦ 116;
    Ascii117 ↦ 117;
    Ascii118 ↦ 118;
    Ascii119 ↦ 119;
    Ascii120 ↦ 120;
    Ascii121 ↦ 121;
    Ascii122 ↦ 122;
    Ascii123 ↦ 123;
    Ascii124 ↦ 124;
    Ascii125 ↦ 125;
    Ascii126 ↦ 126;
    Ascii127 ↦ 127
  }

pub data AsciiCode (n : Int) : Type where {
  MkAsciiCode : (tag : AsciiTag) → Equal Int n (ascii_value tag) → AsciiCode n
}

pub data AllAsciiCodes : List Int → Type where {
  NoCodes : AllAsciiCodes (Nil Int);
  SomeCodes :
    (n : Int)
    → (tail : List Int)
    → AsciiCode n
    → AllAsciiCodes tail
    → AllAsciiCodes (Cons Int n tail)
}

export MkAsciiCode, NoCodes, SomeCodes

pub fn AllAscii (text : String) : Type =
  AllAsciiCodes (map Char Int charToInt (string_to_list_char text))

pub fn AsciiBytes (bs : Bytes) : Type =
  AllAsciiCodes (map UInt8 Int uint8_to_int (bytes_to_list bs))

pub fn IsUtf8 (bs : Bytes) : Prop =
  match bytes_decode bs {
    Err _ ↦ Bottom;
    Ok text ↦ Equal Bytes (bytes_encode text) bs
  }
```

`IsUtf8` is definitionally the existing `Capability.Parsing.Parsing.IsUtf8`:
that package need not import this module or change its `Source` contract.

## 2. Four primitive contracts

The four axioms below are the entire trusted-base delta of this entry. F1
characterizes concatenation in source order. F2 states one whole-ASCII-string
code-list equation, not a per-token table or an encode-over-append premise.
F3 has a proof inhabitant for every `String`; it does not merely postulate an
uninhabited name for a law. F4' covers the ASCII byte lists used by clients,
not arbitrary valid UTF-8 or arbitrary decoded bytes.

```ken
axiom bytes_concat_list_view
    : (a : Bytes)
      → (b : Bytes)
      → Equal
        (List UInt8)
        (bytes_to_list (bytes_concat a b))
        (list_append UInt8 (bytes_to_list a) (bytes_to_list b))

axiom bytes_encode_ascii_octets
    : (s : String)
      → AllAscii s
      → Equal
        (List Int)
        (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s)))
        (map Char Int charToInt (string_to_list_char s))

axiom bytes_decode_encode
    : (s : String)
      → Equal (Result Utf8Error String) (bytes_decode (bytes_encode s)) (Ok Utf8Error String s)

axiom ascii_bytes_utf8 : (bs : Bytes) → AsciiBytes bs → IsUtf8 bs

export bytes_concat_list_view, bytes_encode_ascii_octets, bytes_decode_encode, ascii_bytes_utf8
```

## 3. Trust and derivation

The ledger adds exactly four `Decl::Opaque` postulates:
`bytes_concat_list_view`, `bytes_encode_ascii_octets`,
`bytes_decode_encode`, and `ascii_bytes_utf8`. `AsciiCode`, `AllAsciiCodes`,
`AllAscii`, `AsciiBytes`, and `IsUtf8` are checked inductives or transparent
definitions and contribute no postulate or primitive. Registration occurs when
this late module loads, after `Data.Collections.Derived.list_append` has been
loaded; registering F1 in the early `bytes.rs` prelude would refer to a name
that does not yet exist. Primitive behavior is exercised independently against
each axiom in the module's acceptance test. Neither a general UTF-8 model nor a
fifth fact is implied.
