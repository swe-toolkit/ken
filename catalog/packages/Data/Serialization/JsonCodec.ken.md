# `JsonCodec` — ordered JSON token encoding and decoding

This pure codec translates the existing `Json` value carrier to a structural
token stream and back. The token representation leaves character encoding,
quoting, and numeric spelling to a separate layer.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

An ordered sequence of tokens exposes the structure of a JSON value without
assuming a text parser or string comparison. Arrays retain element order;
objects retain member order, including repeated keys. The existing `Json`
carrier supplies all six value forms, including integral-number leaves.

## Definition

The public token and error types precede the operations and laws. Private
`JsonFrame` and `JsonStep` carriers precede `json_decode` because its decoder
uses them; the frame transitions and proof machinery follow the public laws.

```ken
import Data.Serialization.Json
  (Json, JsonNull, JsonBool, JsonNumber, JsonString, JsonArray, JsonObject)

import Data.Collections.Derived (list_append)

import Core.Logic.Transport (cong, sym, trans)

pub data JsonToken : Type where {
  TokNull : JsonToken;
  TokBool : Bool → JsonToken;
  TokNumber : Int → JsonToken;
  TokString : String → JsonToken;
  TokArrayOpen : JsonToken;
  TokArrayClose : JsonToken;
  TokObjectOpen : JsonToken;
  TokObjectClose : JsonToken;
  TokKey : String → JsonToken
}

export TokNull,
  TokBool,
  TokNumber,
  TokString,
  TokArrayOpen,
  TokArrayClose,
  TokObjectOpen,
  TokObjectClose,
  TokKey

pub data JsonError : Type where {
  UnexpectedEnd : JsonError;
  UnexpectedToken : JsonToken → JsonError;
  MissingKey : JsonError;
  TrailingToken : JsonToken → JsonError
}

export UnexpectedEnd, UnexpectedToken, MissingKey, TrailingToken

data JsonFrame : Type where {
  FrameArray : List Json → JsonFrame;
  FrameObject : List (Pair String Json) → JsonFrame;
  FrameKey : String → List (Pair String Json) → JsonFrame
}

data JsonStep : Type where {
  StepFail : JsonError → JsonStep;
  StepPush : List JsonFrame → JsonStep;
  StepDone : Json → JsonStep
}

pub fn json_encode (value : Json) : List JsonToken =
  match value {
    JsonNull ↦ Cons JsonToken TokNull (Nil JsonToken);
    JsonBool flag ↦ Cons JsonToken (TokBool flag) (Nil JsonToken);
    JsonNumber number ↦ Cons JsonToken (TokNumber number) (Nil JsonToken);
    JsonString text ↦ Cons JsonToken (TokString text) (Nil JsonToken);
    JsonArray values ↦
      Cons
        JsonToken
        TokArrayOpen
        (match values {
          Nil ↦ Cons JsonToken TokArrayClose (Nil JsonToken);
          Cons child rest ↦
            list_append JsonToken (recursive result for child) (recursive result for rest)
        });
    JsonObject members ↦
      Cons
        JsonToken
        TokObjectOpen
        (match members {
          Nil ↦ Cons JsonToken TokObjectClose (Nil JsonToken);
          Cons member rest ↦
            Cons
              JsonToken
              (TokKey (pair_fst String Json member))
              (list_append JsonToken (recursive result for member) (recursive result for rest))
        })
  }

pub fn json_decode (tokens : List JsonToken) : Result JsonError Json =
  json_run tokens (Nil JsonFrame)

pub theorem json_decode_encode
      (value : Json)
    : Equal
        (Result JsonError Json)
        (json_decode (json_encode value))
        (Ok JsonError Json value) =
  trans
    (Result JsonError Json)
    (json_run (json_encode value) (Nil JsonFrame))
    (json_run (list_append JsonToken (json_encode value) (Nil JsonToken)) (Nil JsonFrame))
    (Ok JsonError Json value)
    (cong
      (List JsonToken)
      (Result JsonError Json)
      (json_encode value)
      (list_append JsonToken (json_encode value) (Nil JsonToken))
      (run_with (Nil JsonFrame))
      (sym
        (List JsonToken)
        (list_append JsonToken (json_encode value) (Nil JsonToken))
        (json_encode value)
        ((proof right_unit for list_append) JsonToken (json_encode value))))
    (encode_run
      (Suc (token_count (json_encode value)))
      value
      (Nil JsonToken)
      (Nil JsonFrame)
      (within_count (json_encode value)))

pub theorem json_decode_trailing
      (value : Json) (extra : JsonToken) (more : List JsonToken)
    : Equal
        (Result JsonError Json)
        (json_decode (list_append JsonToken (json_encode value) (Cons JsonToken extra more)))
        (Err JsonError Json (TrailingToken extra)) =
  encode_run
    (Suc (token_count (json_encode value)))
    value
    (Cons JsonToken extra more)
    (Nil JsonFrame)
    (within_count (json_encode value))

pub theorem json_decode_empty
    : Equal
        (Result JsonError Json)
        (json_decode (Nil JsonToken))
        (Err JsonError Json UnexpectedEnd) =
  Proved

pub theorem json_decode_stray_close
      (more : List JsonToken)
    : Equal
        (Result JsonError Json)
        (json_decode (Cons JsonToken TokArrayClose more))
        (Err JsonError Json (UnexpectedToken TokArrayClose)) =
  Proved

fn token_tail (tokens : List JsonToken) : List JsonToken =
  match tokens {
    Nil ↦ Nil JsonToken;
    Cons head rest ↦ rest
  }

fn array_items (values : List Json) : List JsonToken =
  token_tail (json_encode (JsonArray values))

fn object_items (members : List (Pair String Json)) : List JsonToken =
  token_tail (json_encode (JsonObject members))

fn json_complete (value : Json) (stack : List JsonFrame) : JsonStep =
  match stack {
    Nil ↦ StepDone value;
    Cons frame below ↦
      match frame {
        FrameArray items ↦
          StepPush
            (Cons
              JsonFrame
              (FrameArray (list_append Json items (Cons Json value (Nil Json))))
              below);
        FrameObject members ↦ StepFail MissingKey;
        FrameKey key members ↦
          StepPush
            (Cons
              JsonFrame
              (FrameObject
                (list_append
                  (Pair String Json)
                  members
                  (Cons
                    (Pair String Json)
                    (mk_pair String Json key value)
                    (Nil (Pair String Json)))))
              below)
      }
  }

fn json_close_array (stack : List JsonFrame) : JsonStep =
  match stack {
    Nil ↦ StepFail (UnexpectedToken TokArrayClose);
    Cons frame below ↦
      match frame {
        FrameArray items ↦ json_complete (JsonArray items) below;
        FrameObject members ↦ StepFail (UnexpectedToken TokArrayClose);
        FrameKey key members ↦ StepFail (UnexpectedToken TokArrayClose)
      }
  }

fn json_close_object (stack : List JsonFrame) : JsonStep =
  match stack {
    Nil ↦ StepFail (UnexpectedToken TokObjectClose);
    Cons frame below ↦
      match frame {
        FrameArray items ↦ StepFail (UnexpectedToken TokObjectClose);
        FrameObject members ↦ json_complete (JsonObject members) below;
        FrameKey key members ↦ StepFail (UnexpectedToken TokObjectClose)
      }
  }

fn json_open_key (key : String) (stack : List JsonFrame) : JsonStep =
  match stack {
    Nil ↦ StepFail (UnexpectedToken (TokKey key));
    Cons frame below ↦
      match frame {
        FrameArray items ↦ StepFail (UnexpectedToken (TokKey key));
        FrameObject members ↦ StepPush (Cons JsonFrame (FrameKey key members) below);
        FrameKey other members ↦ StepFail (UnexpectedToken (TokKey key))
      }
  }

fn json_step (token : JsonToken) (stack : List JsonFrame) : JsonStep =
  match token {
    TokNull ↦ json_complete JsonNull stack;
    TokBool flag ↦ json_complete (JsonBool flag) stack;
    TokNumber number ↦ json_complete (JsonNumber number) stack;
    TokString text ↦ json_complete (JsonString text) stack;
    TokArrayOpen ↦ StepPush (Cons JsonFrame (FrameArray (Nil Json)) stack);
    TokArrayClose ↦ json_close_array stack;
    TokObjectOpen ↦ StepPush (Cons JsonFrame (FrameObject (Nil (Pair String Json))) stack);
    TokObjectClose ↦ json_close_object stack;
    TokKey key ↦ json_open_key key stack
  }

fn json_finish (tokens : List JsonToken) (value : Json) : Result JsonError Json =
  match tokens {
    Nil ↦ Ok JsonError Json value;
    Cons extra rest ↦ Err JsonError Json (TrailingToken extra)
  }

fn json_run (tokens : List JsonToken) (stack : List JsonFrame) : Result JsonError Json =
  match tokens {
    Nil ↦ Err JsonError Json UnexpectedEnd;
    Cons token rest ↦
      match json_step token stack {
        StepFail error ↦ Err JsonError Json error;
        StepPush next ↦ json_run rest next;
        StepDone value ↦ json_finish rest value
      }
  }

fn json_resume (step : JsonStep) (rest : List JsonToken) : Result JsonError Json =
  match step {
    StepFail error ↦ Err JsonError Json error;
    StepPush next ↦ json_run rest next;
    StepDone value ↦ json_finish rest value
  }

fn within (n : Nat) (tokens : List JsonToken) : Bool =
  match tokens {
    Nil ↦ True;
    Cons head rest ↦
      match n {
        Zero ↦ False;
        Suc m ↦ within m rest
      }
  }

theorem within_cons_append_left
      (m : Nat) (x : JsonToken) (y : JsonToken) (xs : List JsonToken) (ys : List JsonToken)
    : Equal Bool (within m (Cons JsonToken x (list_append JsonToken xs ys))) True
      → Equal Bool (within m (Cons JsonToken y xs)) True =
  match m {
    Zero ↦ λfits. absurd fits;
    Suc k ↦ λfits. within_append_left k xs ys fits
  }

theorem within_cons_append_right
      (m : Nat) (x : JsonToken) (xs : List JsonToken) (ys : List JsonToken)
    : Equal Bool (within m (Cons JsonToken x (list_append JsonToken xs ys))) True
      → Equal Bool (within m (Cons JsonToken x ys)) True =
  match m {
    Zero ↦ λfits. absurd fits;
    Suc k ↦ λfits. within_append_right k xs ys fits
  }

theorem within_drop_second
      (m : Nat) (x : JsonToken) (y : JsonToken) (rest : List JsonToken)
    : Equal Bool (within m (Cons JsonToken x (Cons JsonToken y rest))) True
      → Equal Bool (within m (Cons JsonToken x rest)) True =
  match m {
    Zero ↦ λfits. absurd fits;
    Suc k ↦ λfits. within_tail k y rest fits
  }

fn token_count (tokens : List JsonToken) : Nat =
  match tokens {
    Nil ↦ Zero;
    Cons head rest ↦ Suc (token_count rest)
  }

theorem within_count
      (tokens : List JsonToken)
    : Equal Bool (within (token_count tokens) tokens) True =
  match tokens {
    Nil ↦ Proved;
    Cons head rest ↦ within_count rest
  }

theorem within_suc
      (n : Nat) (tokens : List JsonToken)
    : Equal Bool (within n tokens) True → Equal Bool (within (Suc n) tokens) True =
  match tokens {
    Nil ↦ λfits. Proved;
    Cons head rest ↦
      match n {
        Zero ↦ λfits. absurd fits;
        Suc m ↦ λfits. within_suc m rest fits
      }
  }

theorem within_append_right
      (n : Nat) (xs : List JsonToken) (ys : List JsonToken)
    : Equal Bool (within n (list_append JsonToken xs ys)) True → Equal Bool (within n ys) True =
  match xs {
    Nil ↦ λfits. fits;
    Cons head rest ↦
      match n {
        Zero ↦ λfits. absurd fits;
        Suc m ↦ λfits. within_suc m ys (within_append_right m rest ys fits)
      }
  }

theorem within_append_left
      (n : Nat) (xs : List JsonToken) (ys : List JsonToken)
    : Equal Bool (within n (list_append JsonToken xs ys)) True → Equal Bool (within n xs) True =
  match xs {
    Nil ↦ λfits. Proved;
    Cons head rest ↦
      match n {
        Zero ↦ λfits. absurd fits;
        Suc m ↦ λfits. within_append_left m rest ys fits
      }
  }

theorem within_tail
      (n : Nat) (head : JsonToken) (tokens : List JsonToken)
    : Equal Bool (within n (Cons JsonToken head tokens)) True
      → Equal Bool (within n tokens) True =
  match n {
    Zero ↦ λfits. absurd fits;
    Suc m ↦ λfits. within_suc m tokens fits
  }

theorem acc_shift_json
      (acc : List Json) (child : Json) (more : List Json)
    : Equal
        (List Json)
        (list_append Json (list_append Json acc (Cons Json child (Nil Json))) more)
        (list_append Json acc (Cons Json child more)) =
  (proof assoc for list_append) Json acc (Cons Json child (Nil Json)) more

theorem acc_shift_members
      (acc : List (Pair String Json))
      (member : Pair String Json)
      (more : List (Pair String Json))
    : Equal
        (List (Pair String Json))
        (list_append
          (Pair String Json)
          (list_append
            (Pair String Json)
            acc
            (Cons (Pair String Json) member (Nil (Pair String Json))))
          more)
        (list_append (Pair String Json) acc (Cons (Pair String Json) member more)) =
  (proof assoc for list_append)
    (Pair String Json)
    acc
    (Cons (Pair String Json) member (Nil (Pair String Json)))
    more

fn run_with (stack : List JsonFrame) (tokens : List JsonToken) : Result JsonError Json =
  json_run tokens stack

fn resume_array
      (stack : List JsonFrame) (rest : List JsonToken) (items : List Json)
    : Result JsonError Json =
  json_resume (json_complete (JsonArray items) stack) rest

fn resume_object
      (stack : List JsonFrame) (rest : List JsonToken) (members : List (Pair String Json))
    : Result JsonError Json =
  json_resume (json_complete (JsonObject members) stack) rest

fn encode_runs (n : Nat) : Prop =
  (value : Json)
    → (rest : List JsonToken)
    → (stack : List JsonFrame)
    → Equal Bool
    (within n (Cons JsonToken TokNull (json_encode value)))
    True → Equal
    (Result JsonError Json)
    (json_run (list_append JsonToken (json_encode value) rest) stack)
    (json_resume (json_complete value stack) rest)

theorem array_items_run
      (m : Nat)
      (h : encode_runs m)
      (values : List Json)
      (acc : List Json)
      (rest : List JsonToken)
      (stack : List JsonFrame)
    : Equal Bool (within m (Cons JsonToken TokArrayOpen (array_items values))) True
      → Equal
        (Result JsonError Json)
        (json_run
          (list_append JsonToken (array_items values) rest)
          (Cons JsonFrame (FrameArray acc) stack))
        (resume_array stack rest (list_append Json acc values)) =
  match values {
    Nil ↦
      λfits.
        cong
          (List Json)
          (Result JsonError Json)
          acc
          (list_append Json acc (Nil Json))
          (resume_array stack rest)
          (sym
            (List Json)
            (list_append Json acc (Nil Json))
            acc
            ((proof right_unit for list_append) Json acc));
    Cons child more ↦
      λfits.
        trans
          (Result JsonError Json)
          (json_run
            (list_append
              JsonToken
              (list_append JsonToken (json_encode child) (array_items more))
              rest)
            (Cons JsonFrame (FrameArray acc) stack))
          (json_run
            (list_append
              JsonToken
              (json_encode child)
              (list_append JsonToken (array_items more) rest))
            (Cons JsonFrame (FrameArray acc) stack))
          (resume_array stack rest (list_append Json acc (Cons Json child more)))
          (cong
            (List JsonToken)
            (Result JsonError Json)
            (list_append
              JsonToken
              (list_append JsonToken (json_encode child) (array_items more))
              rest)
            (list_append
              JsonToken
              (json_encode child)
              (list_append JsonToken (array_items more) rest))
            (run_with (Cons JsonFrame (FrameArray acc) stack))
            ((proof assoc for list_append)
              JsonToken
              (json_encode child)
              (array_items more)
              rest))
          (trans
            (Result JsonError Json)
            (json_run
              (list_append
                JsonToken
                (json_encode child)
                (list_append JsonToken (array_items more) rest))
              (Cons JsonFrame (FrameArray acc) stack))
            (json_run
              (list_append JsonToken (array_items more) rest)
              (Cons
                JsonFrame
                (FrameArray (list_append Json acc (Cons Json child (Nil Json))))
                stack))
            (resume_array stack rest (list_append Json acc (Cons Json child more)))
            (h
              child
              (list_append JsonToken (array_items more) rest)
              (Cons JsonFrame (FrameArray acc) stack)
              (within_cons_append_left
                m
                TokArrayOpen
                TokNull
                (json_encode child)
                (array_items more)
                fits))
            (trans
              (Result JsonError Json)
              (json_run
                (list_append JsonToken (array_items more) rest)
                (Cons
                  JsonFrame
                  (FrameArray (list_append Json acc (Cons Json child (Nil Json))))
                  stack))
              (resume_array
                stack
                rest
                (list_append Json (list_append Json acc (Cons Json child (Nil Json))) more))
              (resume_array stack rest (list_append Json acc (Cons Json child more)))
              (array_items_run
                m
                h
                more
                (list_append Json acc (Cons Json child (Nil Json)))
                rest
                stack
                (within_cons_append_right
                  m
                  TokArrayOpen
                  (json_encode child)
                  (array_items more)
                  fits))
              (cong
                (List Json)
                (Result JsonError Json)
                (list_append Json (list_append Json acc (Cons Json child (Nil Json))) more)
                (list_append Json acc (Cons Json child more))
                (resume_array stack rest)
                (acc_shift_json acc child more))))
  }

theorem object_items_run
      (m : Nat)
      (h : encode_runs m)
      (members : List (Pair String Json))
      (acc : List (Pair String Json))
      (rest : List JsonToken)
      (stack : List JsonFrame)
    : Equal Bool (within m (Cons JsonToken TokObjectOpen (object_items members))) True
      → Equal
        (Result JsonError Json)
        (json_run
          (list_append JsonToken (object_items members) rest)
          (Cons JsonFrame (FrameObject acc) stack))
        (resume_object stack rest (list_append (Pair String Json) acc members)) =
  match members {
    Nil ↦
      λfits.
        cong
          (List (Pair String Json))
          (Result JsonError Json)
          acc
          (list_append (Pair String Json) acc (Nil (Pair String Json)))
          (resume_object stack rest)
          (sym
            (List (Pair String Json))
            (list_append (Pair String Json) acc (Nil (Pair String Json)))
            acc
            ((proof right_unit for list_append) (Pair String Json) acc));
    Cons member more ↦
      λfits.
        trans
          (Result JsonError Json)
          (json_run
            (list_append
              JsonToken
              (list_append
                JsonToken
                (json_encode (pair_snd String Json member))
                (object_items more))
              rest)
            (Cons JsonFrame (FrameKey (pair_fst String Json member) acc) stack))
          (json_run
            (list_append
              JsonToken
              (json_encode (pair_snd String Json member))
              (list_append JsonToken (object_items more) rest))
            (Cons JsonFrame (FrameKey (pair_fst String Json member) acc) stack))
          (resume_object
            stack
            rest
            (list_append (Pair String Json) acc (Cons (Pair String Json) member more)))
          (cong
            (List JsonToken)
            (Result JsonError Json)
            (list_append
              JsonToken
              (list_append
                JsonToken
                (json_encode (pair_snd String Json member))
                (object_items more))
              rest)
            (list_append
              JsonToken
              (json_encode (pair_snd String Json member))
              (list_append JsonToken (object_items more) rest))
            (run_with (Cons JsonFrame (FrameKey (pair_fst String Json member) acc) stack))
            ((proof assoc for list_append)
              JsonToken
              (json_encode (pair_snd String Json member))
              (object_items more)
              rest))
          (trans
            (Result JsonError Json)
            (json_run
              (list_append
                JsonToken
                (json_encode (pair_snd String Json member))
                (list_append JsonToken (object_items more) rest))
              (Cons JsonFrame (FrameKey (pair_fst String Json member) acc) stack))
            (json_run
              (list_append JsonToken (object_items more) rest)
              (Cons
                JsonFrame
                (FrameObject
                  (list_append
                    (Pair String Json)
                    acc
                    (Cons (Pair String Json) member (Nil (Pair String Json)))))
                stack))
            (resume_object
              stack
              rest
              (list_append (Pair String Json) acc (Cons (Pair String Json) member more)))
            (h
              (pair_snd String Json member)
              (list_append JsonToken (object_items more) rest)
              (Cons JsonFrame (FrameKey (pair_fst String Json member) acc) stack)
              (within_cons_append_left
                m
                TokObjectOpen
                TokNull
                (json_encode (pair_snd String Json member))
                (object_items more)
                (within_drop_second
                  m
                  TokObjectOpen
                  (TokKey (pair_fst String Json member))
                  (list_append
                    JsonToken
                    (json_encode (pair_snd String Json member))
                    (object_items more))
                  fits)))
            (trans
              (Result JsonError Json)
              (json_run
                (list_append JsonToken (object_items more) rest)
                (Cons
                  JsonFrame
                  (FrameObject
                    (list_append
                      (Pair String Json)
                      acc
                      (Cons (Pair String Json) member (Nil (Pair String Json)))))
                  stack))
              (resume_object
                stack
                rest
                (list_append
                  (Pair String Json)
                  (list_append
                    (Pair String Json)
                    acc
                    (Cons (Pair String Json) member (Nil (Pair String Json))))
                  more))
              (resume_object
                stack
                rest
                (list_append (Pair String Json) acc (Cons (Pair String Json) member more)))
              (object_items_run
                m
                h
                more
                (list_append
                  (Pair String Json)
                  acc
                  (Cons (Pair String Json) member (Nil (Pair String Json))))
                rest
                stack
                (within_cons_append_right
                  m
                  TokObjectOpen
                  (json_encode (pair_snd String Json member))
                  (object_items more)
                  (within_drop_second
                    m
                    TokObjectOpen
                    (TokKey (pair_fst String Json member))
                    (list_append
                      JsonToken
                      (json_encode (pair_snd String Json member))
                      (object_items more))
                    fits)))
              (cong
                (List (Pair String Json))
                (Result JsonError Json)
                (list_append
                  (Pair String Json)
                  (list_append
                    (Pair String Json)
                    acc
                    (Cons (Pair String Json) member (Nil (Pair String Json))))
                  more)
                (list_append (Pair String Json) acc (Cons (Pair String Json) member more))
                (resume_object stack rest)
                (acc_shift_members acc member more))))
  }

theorem encode_run_array
      (m : Nat)
      (h : encode_runs m)
      (values : List Json)
      (rest : List JsonToken)
      (stack : List JsonFrame)
    : Equal Bool (within m (json_encode (JsonArray values))) True
      → Equal
        (Result JsonError Json)
        (json_run (list_append JsonToken (json_encode (JsonArray values)) rest) stack)
        (json_resume (json_complete (JsonArray values) stack) rest) =
  λfits. array_items_run m h values (Nil Json) rest stack fits

theorem encode_run_object
      (m : Nat)
      (h : encode_runs m)
      (members : List (Pair String Json))
      (rest : List JsonToken)
      (stack : List JsonFrame)
    : Equal Bool (within m (json_encode (JsonObject members))) True
      → Equal
        (Result JsonError Json)
        (json_run (list_append JsonToken (json_encode (JsonObject members)) rest) stack)
        (json_resume (json_complete (JsonObject members) stack) rest) =
  λfits. object_items_run m h members (Nil (Pair String Json)) rest stack fits

theorem encode_run_step
      (value : Json)
    : (m : Nat)
      → encode_runs m
      → (rest : List JsonToken)
      → (stack : List JsonFrame)
      → Equal Bool (within m (json_encode value)) True
      → Equal
        (Result JsonError Json)
        (json_run (list_append JsonToken (json_encode value) rest) stack)
        (json_resume (json_complete value stack) rest) =
  match value {
    JsonNull ↦ λm. λh. λrest. λstack. λfits. Refl;
    JsonBool flag ↦ λm. λh. λrest. λstack. λfits. Refl;
    JsonNumber number ↦ λm. λh. λrest. λstack. λfits. Refl;
    JsonString text ↦ λm. λh. λrest. λstack. λfits. Refl;
    JsonArray values ↦ λm. λh. λrest. λstack. encode_run_array m h values rest stack;
    JsonObject members ↦ λm. λh. λrest. λstack. encode_run_object m h members rest stack
  }

theorem encode_run (n : Nat) : encode_runs n =
  match n {
    Zero ↦ λvalue. λrest. λstack. λfits. absurd fits;
    Suc m ↦ λvalue. encode_run_step value m (encode_run m)
  }
```

## Using it

`json_encode` emits one token for a scalar. An array starts with
`TokArrayOpen`, then each child's encoding in order, and ends with
`TokArrayClose`. An object uses `TokKey` before each member value; a key
is not a JSON string value. `json_decode` returns `Ok` for one complete root
or a typed `JsonError` for malformed or trailing input. A client can import
both public types, their exported constructors, the two operations, and the
four laws.

## Laws & proofs

`json_decode_encode` covers the full `Json` carrier, including nested arrays
and objects. `json_decode_trailing` identifies the first token after any
complete encoded root, for every remaining suffix. The empty-input and
stray-array-close laws specify two more distinct errors. These are checked
public theorem bodies in the Definition fence, not observations of finite
examples.

The proof follows a bound on the encoded token count. `encode_run` proves
that running an encoded value followed by a suffix resumes after completing
that value on any stack. Array and object lemmas apply it to each child,
then use `list_append` associativity to align the rest of the stream and
right identity for the empty tail. `cong`, `sym`, and `trans` assemble the
result without examining a `String` or `Int` leaf.

## Design notes

The private stack holds array items, object members, or a pending object key.
Completed items append to each accumulator, preserving the original order and
every repeated key. A value inside an object with no pending key yields
`MissingKey`. Unmatched closing tokens yield `UnexpectedToken`, unfinished
input yields `UnexpectedEnd`, and any token after a completed root yields
`TrailingToken` carrying the first extra token. Error offsets and text or
byte representation belong to a separate codec.

## References

- **RFC 8259, The JavaScript Object Notation Data Interchange Format** —
  https://www.rfc-editor.org/rfc/rfc8259 — the JSON value categories and
  duplicate-member interoperability considerations. This token layer is not
  an implementation of that document's character syntax.
- **JSON — Wikipedia** — https://en.wikipedia.org/wiki/JSON — orientation
  on the value model and structured serialization.
- **Ken serialization contract** — `spec/50-stdlib/71-serialization.md` §1 —
  the token alphabet, typed errors, and four proved obligations.

## Trust & derivation

**Public API:** `JsonToken` and its nine exported constructors, `JsonError`
and its four exported constructors, `json_encode`, `json_decode`, and the
four `json_decode_*` theorems. `JsonFrame`, `JsonStep`, their constructors,
and all decoder and proof helpers are private.

| Reader task | Section |
|---|---|
| Find public operations and their implementation | [Definition](#definition) |
| Learn the token vocabulary | [Using it](#using-it) |
| Review the round trip and refusals | [Laws & proofs](#laws--proofs) |
| Inspect trust posture | [Trust & derivation](#trust--derivation) |

**Derivation:** the codec uses the existing `Data.Serialization.Json` carrier,
`Data.Collections.Derived` list append and its checked laws, and
`Core.Logic.Transport` equality combinators. Its private nested folds and
stack interpreter are pure structural definitions. It introduces no axiom,
postulate, primitive, or opaque assumption; `trusted_base()` must remain
unchanged when the module loads. This delta does not erase any assumptions
inherited from its provider closure.

**Validation:** `conformance/stdlib/serialization/seed-json-token-codec.md`
specifies client-side constructor cases, token order and typed errors. The
package's scoped acceptance target checks public law types, that seed's
supported positive and negative cases, and exact trusted-base identities.
