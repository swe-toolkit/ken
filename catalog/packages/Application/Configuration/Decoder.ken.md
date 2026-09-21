# Application.Configuration.Decoder

`Application.Configuration.Decoder` specializes the shared schema to raw environment and config
key/value pairs. Acquisition remains outside the package: process environment
pairs come from `Capability.Process.Environment`, while config pairs are handed in by the
caller. Keys and values remain plain `Bytes` throughout.

## 1. Local provenance and values

```ken
import Application.Input.Schema
  (MkSchemaIssue,
    Schema,
    SchemaField,
    SchemaFieldCheck,
    SchemaIssue,
    SchemaValidation,
    schema_check_presence,
    schema_field_accept,
    schema_field_name,
    schema_field_presence,
    schema_fields,
    schema_help,
    schema_issue_code,
    schema_issue_origin,
    schema_validate)

import Capability.Diagnostics.Core
  (ConfigKeyOrigin, Diagnostic, EnvironmentOrigin, MkDiagnostic, MkDiagnosticCode, Origin)

import Capability.Formatting.Doc (Doc)

import Capability.Process.Environment (process_environment)

import Core.Classes.LawfulClasses (bytes_deceq_eq)

import Data.Collections.NonEmpty (NonEmpty, nonempty_map)

import Data.Sums.Validation (Invalid, Valid, Validation)

data EnvConfigOrigin = EnvVariableOrigin String | ConfigEntryOrigin (List String)

fn env_config_origin_to_origin (origin : EnvConfigOrigin) : Origin =
  match origin {
    EnvVariableOrigin name ↦ EnvironmentOrigin name;
    ConfigEntryOrigin path ↦ ConfigKeyOrigin path
  }

fn env_config_issue_diagnostic (issue : SchemaIssue EnvConfigOrigin) : Diagnostic =
  MkDiagnostic
    (env_config_origin_to_origin (schema_issue_origin EnvConfigOrigin issue))
    (MkDiagnosticCode (schema_issue_code EnvConfigOrigin issue))
```

## 2. Plain-Bytes lookup

The landed lawful byte dictionary supplies the key decision. No cached length,
string decoding, or client-local equality is involved.

```ken
fn env_config_entry_key (entry : Prod Bytes Bytes) : Bytes =
  match entry {
    MkProd key value ↦ key
  }

fn env_config_entry_value (entry : Prod Bytes Bytes) : Bytes =
  match entry {
    MkProd key value ↦ value
  }

fn env_config_lookup_choice
      (key : Bytes) (entry : Prod Bytes Bytes) (fallback : Option Bytes)
    : Option Bytes =
  match bytes_deceq_eq (env_config_entry_key entry) key {
    True ↦ Some Bytes (env_config_entry_value entry);
    False ↦ fallback
  }

pub fn env_config_lookup (key : Bytes) (entries : List (Prod Bytes Bytes)) : Option Bytes =
  match entries {
    Nil ↦ None Bytes;
    Cons entry rest ↦ env_config_lookup_choice key entry (env_config_lookup key rest)
  }

fn env_config_missing_field
      (origin : EnvConfigOrigin) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin Bool =
  schema_check_presence
    EnvConfigOrigin
    Bool
    True
    (MkSchemaIssue EnvConfigOrigin origin "missing-required-field")
    (schema_field_presence field)

fn env_config_field_check
      (origin : EnvConfigOrigin) (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin Bool =
  match env_config_lookup (bytes_encode (schema_field_name field)) entries {
    Some value ↦ schema_field_accept EnvConfigOrigin Bool True;
    None ↦ env_config_missing_field origin field
  }

fn environment_field_origin (name : String) : EnvConfigOrigin = EnvVariableOrigin name

fn config_field_origin (name : String) : EnvConfigOrigin =
  ConfigEntryOrigin (Cons String name (Nil String))

fn environment_field_check
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin Bool =
  env_config_field_check (environment_field_origin (schema_field_name field)) entries field

fn config_field_check
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin Bool =
  env_config_field_check (config_field_origin (schema_field_name field)) entries field

fn env_config_values
      (fields : List SchemaField) (entries : List (Prod Bytes Bytes))
    : List Bytes =
  match fields {
    Nil ↦ Nil Bytes;
    Cons field rest ↦
      match env_config_lookup (bytes_encode (schema_field_name field)) entries {
        Some value ↦ Cons Bytes value (env_config_values rest entries);
        None ↦ Cons Bytes (list_to_bytes (Nil UInt8)) (env_config_values rest entries)
      }
  }

theorem env_config_some_injective
      (a : Type) (left : a) (right : a) (same : Equal (Option a) (Some a left) (Some a right))
    : Equal a left right =
  same

theorem env_config_valid_injective
      (e : Type)
      (a : Type)
      (left : a)
      (right : a)
      (same : Equal (Validation e a) (Valid e a left) (Valid e a right))
    : Equal a left right =
  same

theorem env_config_lookup_field_transport
      (head : SchemaField)
      (field : SchemaField)
      (entries : List (Prod Bytes Bytes))
      (choice : Option Bytes)
      (same : Equal SchemaField head field)
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) entries)
        choice)
    : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name head)) entries)
        choice =
  J
    (λfield2 _.
      Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field2)) entries)
        choice
      → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name head)) entries)
        choice)
    (λhlookup2. hlookup2)
    same
    hlookup

theorem env_config_sym
      (a : Type) (left : a) (right : a) (same : Equal a left right)
    : Equal a right left =
  J (λactual _. Equal a actual left) Refl same

theorem env_config_trans
      (a : Type)
      (left : a)
      (middle : a)
      (right : a)
      (first : Equal a left middle)
      (second : Equal a middle right)
    : Equal a left right =
  J (λright2 _. Equal a left right2) first second

theorem env_config_lookup_cases
      (key : Bytes) (entries : List (Prod Bytes Bytes))
    : (goal : Prop)
      → (Equal (Option Bytes) (env_config_lookup key entries) (None Bytes) → goal)
      → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup key entries)
          (Some Bytes value)
          → goal)
      → goal =
  match env_config_lookup key entries {
    None ↦ λgoal. λrecover_none. λrecover_some. recover_none Proved;
    Some value ↦ λgoal. λrecover_none. λrecover_some. recover_some value Refl
  }

theorem env_config_values_tail
      (head : SchemaField)
      (rest : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (i : Nat)
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth
          Bytes
          (Suc i)
          (env_config_values (Cons SchemaField head rest) entries))
        (Data.Collections.Derived.nth Bytes i (env_config_values rest entries)) =
  env_config_lookup_cases
    (bytes_encode (schema_field_name head))
    entries
    (Equal
      (Option Bytes)
      (Data.Collections.Derived.nth
        Bytes
        (Suc i)
        (env_config_values (Cons SchemaField head rest) entries))
      (Data.Collections.Derived.nth Bytes i (env_config_values rest entries)))
    (λhlookup.
      J
        (λchoice _.
          Equal
            (Option Bytes)
            (Data.Collections.Derived.nth
              Bytes
              (Suc i)
              (match choice {
                None ↦ Cons Bytes (list_to_bytes (Nil UInt8)) (env_config_values rest entries);
                Some value ↦ Cons Bytes value (env_config_values rest entries)
              }))
            (Data.Collections.Derived.nth Bytes i (env_config_values rest entries)))
        Refl
        (env_config_sym
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name head)) entries)
          (None Bytes)
          hlookup))
    (λvalue.
      λhlookup.
        J
          (λchoice _.
            Equal
              (Option Bytes)
              (Data.Collections.Derived.nth
                Bytes
                (Suc i)
                (match choice {
                  None ↦
                    Cons Bytes (list_to_bytes (Nil UInt8)) (env_config_values rest entries);
                  Some found ↦ Cons Bytes found (env_config_values rest entries)
                }))
              (Data.Collections.Derived.nth Bytes i (env_config_values rest entries)))
          Refl
          (env_config_sym
            (Option Bytes)
            (env_config_lookup (bytes_encode (schema_field_name head)) entries)
            (Some Bytes value)
            hlookup))

theorem env_config_head_lookup_some
      (head : SchemaField)
      (rest : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (value : Bytes)
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name head)) entries)
        (Some Bytes value))
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth
          Bytes
          Zero
          (env_config_values (Cons SchemaField head rest) entries))
        (Some Bytes value) =
  J
    (λchoice _.
      Equal
        (Option Bytes)
        (Data.Collections.Derived.nth
          Bytes
          Zero
          (match choice {
            Some found ↦ Cons Bytes found (env_config_values rest entries);
            None ↦ Cons Bytes (list_to_bytes (Nil UInt8)) (env_config_values rest entries)
          }))
        (Some Bytes value))
    Refl
    (env_config_sym
      (Option Bytes)
      (env_config_lookup (bytes_encode (schema_field_name head)) entries)
      (Some Bytes value)
      hlookup)

theorem env_config_head_lookup_none
      (head : SchemaField)
      (rest : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name head)) entries)
        (None Bytes))
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth
          Bytes
          Zero
          (env_config_values (Cons SchemaField head rest) entries))
        (Some Bytes (list_to_bytes (Nil UInt8))) =
  J
    (λchoice _.
      Equal
        (Option Bytes)
        (Data.Collections.Derived.nth
          Bytes
          Zero
          (match choice {
            Some found ↦ Cons Bytes found (env_config_values rest entries);
            None ↦ Cons Bytes (list_to_bytes (Nil UInt8)) (env_config_values rest entries)
          }))
        (Some Bytes (list_to_bytes (Nil UInt8))))
    Refl
    (env_config_sym
      (Option Bytes)
      (env_config_lookup (bytes_encode (schema_field_name head)) entries)
      (None Bytes)
      hlookup)

theorem env_config_required_check_lookup
      (origin : EnvConfigOrigin)
      (entries : List (Prod Bytes Bytes))
      (name : String)
      (presence : Application.Input.Schema.SchemaPresence)
      (shape : Application.Input.Schema.SchemaValueShape)
      (documentation : String)
      (hpresence : Equal
        Application.Input.Schema.SchemaPresence
        presence
        Application.Input.Schema.SchemaRequired)
      (accepted : Bool)
      (haccepted : Equal
        (SchemaFieldCheck EnvConfigOrigin Bool)
        (env_config_field_check
          origin
          entries
          (Application.Input.Schema.MkSchemaField name presence shape documentation))
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted))
      (goal : Prop)
      (recover : (value : Bytes)
        → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode name) entries)
        (Some Bytes value)
        → goal)
    : goal =
  env_config_lookup_cases
    (bytes_encode name)
    entries
    goal
    (λhlookup.
      absurd
        (J
          (λchoice _.
            Equal
              (SchemaFieldCheck EnvConfigOrigin Bool)
              (match choice {
                Some value ↦ schema_field_accept EnvConfigOrigin Bool True;
                None ↦
                  env_config_missing_field
                    origin
                    (Application.Input.Schema.MkSchemaField
                      name
                      Application.Input.Schema.SchemaRequired
                      shape
                      documentation)
              })
              (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted))
          (J
            (λpresence2 _.
              Equal
                (SchemaFieldCheck EnvConfigOrigin Bool)
                (env_config_field_check
                  origin
                  entries
                  (Application.Input.Schema.MkSchemaField name presence2 shape documentation))
                (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted))
            haccepted
            hpresence)
          hlookup))
    (λvalue. λhlookup. recover value hlookup)

proof lookup_some for env_config_values
      (fields : List SchemaField) (entries : List (Prod Bytes Bytes))
    : (i : Nat)
      → (field : SchemaField)
      → (value : Bytes)
      → Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i fields)
        (Some SchemaField field)
      → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) entries)
        (Some Bytes value)
      → Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i (env_config_values fields entries))
        (Some Bytes value) =
  match fields {
    Nil ↦ λi. λfield. λvalue. λhfield. λhlookup. absurd hfield;
    Cons head rest ↦
      λi.
        match i {
          Zero ↦
            λfield.
              λvalue.
                λhfield.
                  λhlookup.
                    env_config_head_lookup_some
                      head
                      rest
                      entries
                      value
                      (env_config_lookup_field_transport
                        head
                        field
                        entries
                        (Some Bytes value)
                        (env_config_some_injective SchemaField head field hfield)
                        hlookup);
          Suc i2 ↦
            λfield.
              λvalue.
                λhfield.
                  λhlookup.
                    env_config_trans
                      (Option Bytes)
                      (Data.Collections.Derived.nth
                        Bytes
                        (Suc i2)
                        (env_config_values (Cons SchemaField head rest) entries))
                      (Data.Collections.Derived.nth Bytes i2 (env_config_values rest entries))
                      (Some Bytes value)
                      (env_config_values_tail head rest entries i2)
                      ((proof lookup_some for env_config_values)
                        rest
                        entries
                        i2
                        field
                        value
                        hfield
                        hlookup)
        }
  }

theorem environment_required_check_lookup
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : (accepted : Bool)
      → Equal
        (SchemaFieldCheck EnvConfigOrigin Bool)
        (environment_field_check entries field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted)
      → Equal Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired
      → (goal : Prop)
      → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name field)) entries)
          (Some Bytes value)
          → goal)
      → goal =
  match field {
    Application.Input.Schema.MkSchemaField name presence shape documentation ↦
      λaccepted.
        λhcheck.
          λhpresence.
            λgoal.
              λrecover.
                env_config_required_check_lookup
                  (environment_field_origin name)
                  entries
                  name
                  presence
                  shape
                  documentation
                  hpresence
                  accepted
                  hcheck
                  goal
                  recover
  }

theorem config_required_check_lookup
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : (accepted : Bool)
      → Equal
        (SchemaFieldCheck EnvConfigOrigin Bool)
        (config_field_check entries field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted)
      → Equal Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired
      → (goal : Prop)
      → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name field)) entries)
          (Some Bytes value)
          → goal)
      → goal =
  match field {
    Application.Input.Schema.MkSchemaField name presence shape documentation ↦
      λaccepted.
        λhcheck.
          λhpresence.
            λgoal.
              λrecover.
                env_config_required_check_lookup
                  (config_field_origin name)
                  entries
                  name
                  presence
                  shape
                  documentation
                  hpresence
                  accepted
                  hcheck
                  goal
                  recover
  }

theorem env_config_required_values
      (inspect : SchemaField → SchemaFieldCheck EnvConfigOrigin Bool)
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (checked_values : List Bool)
      (values : List Bytes)
      (hvalid : Equal
        (SchemaValidation EnvConfigOrigin Bool)
        (Application.Input.Schema.schema_validate_fields EnvConfigOrigin Bool inspect fields)
        (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) checked_values))
      (hvalues : Equal (List Bytes) (env_config_values fields entries) values)
      (accepted_lookup : (field : SchemaField)
        → (accepted : Bool)
        → Equal
        (SchemaFieldCheck EnvConfigOrigin Bool)
        (inspect field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin Bool accepted)
        → Equal
        Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired
        → (goal : Prop)
        → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name field)) entries)
          (Some Bytes value)
          → goal)
        → goal)
    : (i : Nat)
      → (field : SchemaField)
      → Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i fields)
        (Some SchemaField field)
      → Equal Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired
      → (goal : Prop)
      → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name field)) entries)
          (Some Bytes value)
          → Equal
          (Option Bytes)
          (Data.Collections.Derived.nth Bytes i values)
          (Some Bytes value)
          → goal)
      → goal =
  λi.
    λfield.
      λhfield.
        λhpresence.
          λgoal.
            λrecover.
              Application.Input.Schema.schema_validate_fields::valid_coverage
                EnvConfigOrigin
                Bool
                inspect
                fields
                checked_values
                hvalid
                i
                field
                hfield
                goal
                (λaccepted.
                  λhnth.
                    λhcheck.
                      accepted_lookup
                        field
                        accepted
                        hcheck
                        hpresence
                        goal
                        (λvalue.
                          λhlookup.
                            recover
                              value
                              hlookup
                              (J
                                (λvalues2 _.
                                  Equal
                                    (Option Bytes)
                                    (Data.Collections.Derived.nth Bytes i values2)
                                    (Some Bytes value))
                                ((proof lookup_some for env_config_values)
                                  fields
                                  entries
                                  i
                                  field
                                  value
                                  hfield
                                  hlookup)
                                hvalues)))

proof lookup_none for env_config_values
      (fields : List SchemaField) (entries : List (Prod Bytes Bytes))
    : (i : Nat)
      → (field : SchemaField)
      → Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i fields)
        (Some SchemaField field)
      → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) entries)
        (None Bytes)
      → Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i (env_config_values fields entries))
        (Some Bytes (list_to_bytes (Nil UInt8))) =
  match fields {
    Nil ↦ λi. λfield. λhfield. λhlookup. absurd hfield;
    Cons head rest ↦
      λi.
        match i {
          Zero ↦
            λfield.
              λhfield.
                λhlookup.
                  env_config_head_lookup_none
                    head
                    rest
                    entries
                    (env_config_lookup_field_transport
                      head
                      field
                      entries
                      (None Bytes)
                      (env_config_some_injective SchemaField head field hfield)
                      hlookup);
          Suc i2 ↦
            λfield.
              λhfield.
                λhlookup.
                  env_config_trans
                    (Option Bytes)
                    (Data.Collections.Derived.nth
                      Bytes
                      (Suc i2)
                      (env_config_values (Cons SchemaField head rest) entries))
                    (Data.Collections.Derived.nth Bytes i2 (env_config_values rest entries))
                    (Some Bytes (list_to_bytes (Nil UInt8)))
                    (env_config_values_tail head rest entries i2)
                    ((proof lookup_none for env_config_values)
                      rest
                      entries
                      i2
                      field
                      hfield
                      hlookup)
        }
  }

theorem env_config_schema_validation_cases
      (checked : SchemaValidation EnvConfigOrigin Bool)
    : (goal : Prop)
      → ((values : List Bool)
          → Equal
          (SchemaValidation EnvConfigOrigin Bool)
          checked
          (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) values)
          → goal)
      → ((issues : NonEmpty (SchemaIssue EnvConfigOrigin))
          → Equal
          (SchemaValidation EnvConfigOrigin Bool)
          checked
          (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) issues)
          → goal)
      → goal =
  match checked {
    Valid values ↦ λgoal. λrecover_valid. λrecover_invalid. recover_valid values Refl;
    Invalid issues ↦ λgoal. λrecover_valid. λrecover_invalid. recover_invalid issues Refl
  }

theorem env_config_decode_invalid_impossible
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (checked : SchemaValidation EnvConfigOrigin Bool)
      (issues : NonEmpty (SchemaIssue EnvConfigOrigin))
      (values : List Bytes)
      (hchecked : Equal
        (SchemaValidation EnvConfigOrigin Bool)
        checked
        (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) issues))
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (env_config_validation fields entries checked)
        (Valid (NonEmpty Diagnostic) (List Bytes) values))
    : Bottom =
  J
    (λchecked2 _.
      Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (env_config_validation fields entries checked2)
        (Valid (NonEmpty Diagnostic) (List Bytes) values)
      → Bottom)
    (λhdecode2. absurd hdecode2)
    (env_config_sym
      (SchemaValidation EnvConfigOrigin Bool)
      checked
      (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) issues)
      hchecked)
    hdecode

theorem env_config_decode_valid_values
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (checked : SchemaValidation EnvConfigOrigin Bool)
      (checked_values : List Bool)
      (values : List Bytes)
      (hchecked : Equal
        (SchemaValidation EnvConfigOrigin Bool)
        checked
        (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) checked_values))
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (env_config_validation fields entries checked)
        (Valid (NonEmpty Diagnostic) (List Bytes) values))
    : Equal (List Bytes) (env_config_values fields entries) values =
  J
    (λchecked2 _.
      Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (env_config_validation fields entries checked2)
        (Valid (NonEmpty Diagnostic) (List Bytes) values)
      → Equal (List Bytes) (env_config_values fields entries) values)
    (λhdecode2.
      env_config_valid_injective
        (NonEmpty Diagnostic)
        (List Bytes)
        (env_config_values fields entries)
        values
        hdecode2)
    (env_config_sym
      (SchemaValidation EnvConfigOrigin Bool)
      checked
      (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List Bool) checked_values)
      hchecked)
    hdecode

theorem env_config_optional_values
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (values : List Bytes)
      (hvalues : Equal (List Bytes) (env_config_values fields entries) values)
      (i : Nat)
      (field : SchemaField)
      (hfield : Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i fields)
        (Some SchemaField field))
      (hpresence : Equal
        Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaOptional)
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) entries)
        (None Bytes))
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i values)
        (Some Bytes (list_to_bytes (Nil UInt8))) =
  J
    (λvalues2 _.
      Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i values2)
        (Some Bytes (list_to_bytes (Nil UInt8))))
    ((proof lookup_none for env_config_values) fields entries i field hfield hlookup)
    hvalues
```

## 3. Schema-driven decoding

Both entry points run the same shared accumulating traversal. Their origin
construction remains local, and only this specialization injects issues into
`Diagnostic`.

```ken
fn env_config_validation
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (checked : SchemaValidation EnvConfigOrigin Bool)
    : Validation (NonEmpty Diagnostic) (List Bytes) =
  match checked {
    Valid values ↦ Valid (NonEmpty Diagnostic) (List Bytes) (env_config_values fields entries);
    Invalid issues ↦
      Invalid
        (NonEmpty Diagnostic)
        (List Bytes)
        (nonempty_map
          (SchemaIssue EnvConfigOrigin)
          Diagnostic
          env_config_issue_diagnostic
          issues)
  }

fn decode_environment_entries
      (schema : Schema) (entries : List (Prod Bytes Bytes))
    : Validation (NonEmpty Diagnostic) (List Bytes) =
  env_config_validation
    (schema_fields schema)
    entries
    (schema_validate EnvConfigOrigin Bool (environment_field_check entries) schema)

fn decode_process_environment
      (schema : Schema) (input : ProcessInput)
    : Validation (NonEmpty Diagnostic) (List Bytes) =
  decode_environment_entries schema (process_environment input)

fn decode_config_entries
      (schema : Schema) (entries : List (Prod Bytes Bytes))
    : Validation (NonEmpty Diagnostic) (List Bytes) =
  env_config_validation
    (schema_fields schema)
    entries
    (schema_validate EnvConfigOrigin Bool (config_field_check entries) schema)

fn env_config_help (schema : Schema) : Doc = schema_help schema

export decode_process_environment, decode_config_entries, env_config_help
```

## 4. Laws and proofs

The required-field laws instantiate the schema traversal's checked coverage.
They then connect each accepted required field to the exact `env_config_lookup`
result and to the second traversal's aligned output. The value is the selected
raw `Bytes`; no text conversion or re-encoding occurs.

The optional-absence laws separately characterize the predecessor
representation. A valid decode whose optional lookup is `None` carries an empty
`Bytes` value at the aligned index. This describes the existing lossy carrier;
it does not make absence and a present empty value semantically identical.

```ken
pub proof required_lookup for decode_process_environment
      (schema : Schema)
      (input : ProcessInput)
      (values : List Bytes)
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (decode_process_environment schema input)
        (Valid (NonEmpty Diagnostic) (List Bytes) values))
      (i : Nat)
      (field : SchemaField)
      (hfield : Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i (schema_fields schema))
        (Some SchemaField field))
      (hpresence : Equal
        Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired)
      (goal : Prop)
      (recover : (value : Bytes)
        → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) (process_environment input))
        (Some Bytes value)
        → Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i values)
        (Some Bytes value)
        → goal)
    : goal =
  env_config_schema_validation_cases
    (schema_validate
      EnvConfigOrigin
      Bool
      (environment_field_check (process_environment input))
      schema)
    goal
    (λchecked_values.
      λhchecked.
        env_config_required_values
          (environment_field_check (process_environment input))
          (schema_fields schema)
          (process_environment input)
          checked_values
          values
          hchecked
          (env_config_decode_valid_values
            (schema_fields schema)
            (process_environment input)
            (schema_validate
              EnvConfigOrigin
              Bool
              (environment_field_check (process_environment input))
              schema)
            checked_values
            values
            hchecked
            hdecode)
          (environment_required_check_lookup (process_environment input))
          i
          field
          hfield
          hpresence
          goal
          recover)
    (λissues.
      λhchecked.
        absurd
          (env_config_decode_invalid_impossible
            (schema_fields schema)
            (process_environment input)
            (schema_validate
              EnvConfigOrigin
              Bool
              (environment_field_check (process_environment input))
              schema)
            issues
            values
            hchecked
            hdecode))

pub proof optional_absence for decode_process_environment
      (schema : Schema)
      (input : ProcessInput)
      (values : List Bytes)
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (decode_process_environment schema input)
        (Valid (NonEmpty Diagnostic) (List Bytes) values))
      (i : Nat)
      (field : SchemaField)
      (hfield : Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i (schema_fields schema))
        (Some SchemaField field))
      (hpresence : Equal
        Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaOptional)
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) (process_environment input))
        (None Bytes))
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i values)
        (Some Bytes (list_to_bytes (Nil UInt8))) =
  env_config_schema_validation_cases
    (schema_validate
      EnvConfigOrigin
      Bool
      (environment_field_check (process_environment input))
      schema)
    (Equal
      (Option Bytes)
      (Data.Collections.Derived.nth Bytes i values)
      (Some Bytes (list_to_bytes (Nil UInt8))))
    (λchecked_values.
      λhchecked.
        env_config_optional_values
          (schema_fields schema)
          (process_environment input)
          values
          (env_config_decode_valid_values
            (schema_fields schema)
            (process_environment input)
            (schema_validate
              EnvConfigOrigin
              Bool
              (environment_field_check (process_environment input))
              schema)
            checked_values
            values
            hchecked
            hdecode)
          i
          field
          hfield
          hpresence
          hlookup)
    (λissues.
      λhchecked.
        absurd
          (env_config_decode_invalid_impossible
            (schema_fields schema)
            (process_environment input)
            (schema_validate
              EnvConfigOrigin
              Bool
              (environment_field_check (process_environment input))
              schema)
            issues
            values
            hchecked
            hdecode))

pub proof required_lookup for decode_config_entries
      (schema : Schema)
    : (entries : List (Prod Bytes Bytes))
      → (values : List Bytes)
      → Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (decode_config_entries schema entries)
        (Valid (NonEmpty Diagnostic) (List Bytes) values)
      → (i : Nat)
      → (field : SchemaField)
      → Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i (schema_fields schema))
        (Some SchemaField field)
      → Equal Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaRequired
      → (goal : Prop)
      → ((value : Bytes)
          → Equal
          (Option Bytes)
          (env_config_lookup (bytes_encode (schema_field_name field)) entries)
          (Some Bytes value)
          → Equal
          (Option Bytes)
          (Data.Collections.Derived.nth Bytes i values)
          (Some Bytes value)
          → goal)
      → goal =
  λentries.
    λvalues.
      λhdecode.
        λi.
          λfield.
            λhfield.
              λhpresence.
                λgoal.
                  λrecover.
                    env_config_schema_validation_cases
                      (schema_validate EnvConfigOrigin Bool (config_field_check entries) schema)
                      goal
                      (λchecked_values.
                        λhchecked.
                          env_config_required_values
                            (config_field_check entries)
                            (schema_fields schema)
                            entries
                            checked_values
                            values
                            hchecked
                            (env_config_decode_valid_values
                              (schema_fields schema)
                              entries
                              (schema_validate
                                EnvConfigOrigin
                                Bool
                                (config_field_check entries)
                                schema)
                              checked_values
                              values
                              hchecked
                              hdecode)
                            (config_required_check_lookup entries)
                            i
                            field
                            hfield
                            hpresence
                            goal
                            recover)
                      (λissues.
                        λhchecked.
                          absurd
                            (env_config_decode_invalid_impossible
                              (schema_fields schema)
                              entries
                              (schema_validate
                                EnvConfigOrigin
                                Bool
                                (config_field_check entries)
                                schema)
                              issues
                              values
                              hchecked
                              hdecode))

pub proof optional_absence for decode_config_entries
      (schema : Schema)
      (entries : List (Prod Bytes Bytes))
      (values : List Bytes)
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List Bytes))
        (decode_config_entries schema entries)
        (Valid (NonEmpty Diagnostic) (List Bytes) values))
      (i : Nat)
      (field : SchemaField)
      (hfield : Equal
        (Option SchemaField)
        (Data.Collections.Derived.nth SchemaField i (schema_fields schema))
        (Some SchemaField field))
      (hpresence : Equal
        Application.Input.Schema.SchemaPresence
        (schema_field_presence field)
        Application.Input.Schema.SchemaOptional)
      (hlookup : Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode (schema_field_name field)) entries)
        (None Bytes))
    : Equal
        (Option Bytes)
        (Data.Collections.Derived.nth Bytes i values)
        (Some Bytes (list_to_bytes (Nil UInt8))) =
  env_config_schema_validation_cases
    (schema_validate EnvConfigOrigin Bool (config_field_check entries) schema)
    (Equal
      (Option Bytes)
      (Data.Collections.Derived.nth Bytes i values)
      (Some Bytes (list_to_bytes (Nil UInt8))))
    (λchecked_values.
      λhchecked.
        env_config_optional_values
          (schema_fields schema)
          entries
          values
          (env_config_decode_valid_values
            (schema_fields schema)
            entries
            (schema_validate EnvConfigOrigin Bool (config_field_check entries) schema)
            checked_values
            values
            hchecked
            hdecode)
          i
          field
          hfield
          hpresence
          hlookup)
    (λissues.
      λhchecked.
        absurd
          (env_config_decode_invalid_impossible
            (schema_fields schema)
            entries
            (schema_validate EnvConfigOrigin Bool (config_field_check entries) schema)
            issues
            values
            hchecked
            hdecode))
```

## 5. Trust and boundaries

The public surface consists of `decode_process_environment`,
`decode_config_entries`, `env_config_help`, and `env_config_lookup`, together
with the two attached laws on each decoder. The lookup's existing first-match
behavior is the single authority used by both the validation and value
traversals; its three implementation helpers remain private.

The decoder consumes `process_environment`, shared `Schema`, `Validation`,
`Diagnostic`, and the landed lawful `DecEq Bytes`. It adds no parser, renderer,
location carrier, cached length, primitive, postulate, `Axiom`, or trusted-base
entry. Raw values—including invalid UTF-8—are returned without a `String` hop.
