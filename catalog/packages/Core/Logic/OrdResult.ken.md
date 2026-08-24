# `OrdResult` — canonical three-way comparison result

`OrdResult` records whether one value compares below, equal to, or above
another. The named constants avoid collisions between the `Eq` constructor and
other equality vocabulary while preserving the same constructor identities.

## Definition

```ken
data OrdResult = Lt | Eq | Gt

const ord_eq : OrdResult = Eq

const ord_lt : OrdResult = Lt

const ord_gt : OrdResult = Gt

fn ord_result_leq (r : OrdResult) : Bool =
  match r {
    Lt ↦ True;
    Eq ↦ True;
    Gt ↦ False
  }

fn ord_result_dispatch2
      (c : Type)
      (ll : c)
      (le : c)
      (lg : c)
      (el : c)
      (ee : c)
      (eg : c)
      (gl : c)
      (ge : c)
      (gg : c)
      (r : OrdResult)
      (s : OrdResult)
    : c =
  match r {
    Lt ↦
      match s {
        Lt ↦ ll;
        Eq ↦ le;
        Gt ↦ lg
      };
    Eq ↦
      match s {
        Lt ↦ el;
        Eq ↦ ee;
        Gt ↦ eg
      };
    Gt ↦
      match s {
        Lt ↦ gl;
        Eq ↦ ge;
        Gt ↦ gg
      }
  }

theorem ord_result_elim
      (P : OrdResult → Omega) (r : OrdResult) (pLt : P Lt) (pEq : P Eq) (pGt : P Gt)
    : P r =
  match r {
    Lt ↦ pLt;
    Eq ↦ pEq;
    Gt ↦ pGt
  }

theorem ord_result_elim2
      (P : OrdResult → OrdResult → Omega)
      (r : OrdResult)
      (s : OrdResult)
      (pLL : P Lt Lt)
      (pLE : P Lt Eq)
      (pLG : P Lt Gt)
      (pEL : P Eq Lt)
      (pEE : P Eq Eq)
      (pEG : P Eq Gt)
      (pGL : P Gt Lt)
      (pGE : P Gt Eq)
      (pGG : P Gt Gt)
    : P r s =
  match r {
    Lt ↦
      match s {
        Lt ↦ pLL;
        Eq ↦ pLE;
        Gt ↦ pLG
      };
    Eq ↦
      match s {
        Lt ↦ pEL;
        Eq ↦ pEE;
        Gt ↦ pEG
      };
    Gt ↦
      match s {
        Lt ↦ pGL;
        Eq ↦ pGE;
        Gt ↦ pGG
      }
  }

export OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt

export ord_result_leq, ord_result_dispatch2, ord_result_elim, ord_result_elim2
```

The type, constructors, aliases, and eliminators form one public interface.
The declarations are ordinary checked Ken and add no trusted assumption.
