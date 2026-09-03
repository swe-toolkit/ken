# `Data.Binary.BytesKeys` — lawful byte equality

`UInt8` equality transports through the existing widening injection into
`Int`. `Bytes` equality then transports through the landed structural view into
`List UInt8`. The canonical dictionaries and their supporting proofs live with `DecEq` in
`Core.Classes.LawfulClasses`; this compatibility package imports that owner so
loading the historical package path registers the same dictionaries.

## 1. `UInt8` equality

```ken
import Core.Classes.LawfulClasses (DecEq)
```

## 2. `Bytes` equality



## 3. Trust and derivation

This package declares no local trust and no local equality dictionary. The
imported class owner supplies both canonical instances and retains their existing
trust accounting.
