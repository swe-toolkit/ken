# Application.Configuration.Decoder

`Application.Configuration.Decoder` specializes the shared schema to raw
key/value pairs. Acquisition remains outside the package: process environment
pairs come from `Capability.Process.Environment`, while config pairs are handed
in by the caller. Keys and present values remain raw `Bytes`; an absent optional
field has its own `None` result element.

## 1. Decoder interface

The entry points follow the schema's field order. They return a value for
present fields and a distinct absence marker for missing optional fields.

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

import Application.Input.Schema

import Capability.Diagnostics.Core
  (ConfigKeyOrigin, Diagnostic, EnvironmentOrigin, MkDiagnostic, MkDiagnosticCode, Origin)

import Capability.Formatting.Doc (Doc)

import Capability.Process.Environment (process_environment)

import Core.Classes.LawfulClasses (bytes_deceq_eq)

import Core.Logic.Transport (cong, sym, trans)

import Data.Collections.Derived

import Data.Collections.NonEmpty (NonEmpty, nonempty_map)

import Data.Sums.Validation (Invalid, Valid, Validation)

data EnvConfigOrigin = EnvVariableOrigin String | ConfigEntryOrigin (List String)

fn decode_process_environment
      (schema : Schema) (input : ProcessInput)
    : Validation (NonEmpty Diagnostic) (List (Option Bytes)) =
  decode_environment_entries schema (process_environment input)

fn decode_config_entries
      (schema : Schema) (entries : List (Prod Bytes Bytes))
    : Validation (NonEmpty Diagnostic) (List (Option Bytes)) =
  env_config_validation
    (schema_validate EnvConfigOrigin (Option Bytes) (config_field_check entries) schema)

fn env_config_help (schema : Schema) : Doc = schema_help schema

pub fn env_config_lookup (key : Bytes) (entries : List (Prod Bytes Bytes)) : Option Bytes =
  match entries {
    Nil ↦ None Bytes;
    Cons entry rest ↦ env_config_lookup_choice key entry (env_config_lookup key rest)
  }
```

## 2. Raw lookup and origins

The landed lawful byte dictionary supplies the key decision. No cached length,
string decoding, or client-local equality is involved. Both sources retain
their own issue origins.

```ken
fn env_config_origin_to_origin (origin : EnvConfigOrigin) : Origin =
  match origin {
    EnvVariableOrigin name ↦ EnvironmentOrigin name;
    ConfigEntryOrigin path ↦ ConfigKeyOrigin path
  }

fn env_config_issue_diagnostic (issue : SchemaIssue EnvConfigOrigin) : Diagnostic =
  MkDiagnostic
    (env_config_origin_to_origin (schema_issue_origin EnvConfigOrigin issue))
    (MkDiagnosticCode (schema_issue_code EnvConfigOrigin issue))

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

fn env_config_missing_field
      (origin : EnvConfigOrigin) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin (Option Bytes) =
  schema_check_presence
    EnvConfigOrigin
    (Option Bytes)
    (None Bytes)
    (MkSchemaIssue EnvConfigOrigin origin "missing-required-field")
    (schema_field_presence field)

fn env_config_field_check
      (origin : EnvConfigOrigin) (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin (Option Bytes) =
  match env_config_lookup (bytes_encode (schema_field_name field)) entries {
    Some value ↦ schema_field_accept EnvConfigOrigin (Option Bytes) (Some Bytes value);
    None ↦ env_config_missing_field origin field
  }

fn environment_field_origin (name : String) : EnvConfigOrigin = EnvVariableOrigin name

fn config_field_origin (name : String) : EnvConfigOrigin =
  ConfigEntryOrigin (Cons String name (Nil String))

fn environment_field_check
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin (Option Bytes) =
  env_config_field_check (environment_field_origin (schema_field_name field)) entries field

fn config_field_check
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : SchemaFieldCheck EnvConfigOrigin (Option Bytes) =
  env_config_field_check (config_field_origin (schema_field_name field)) entries field
```

## 3. Schema-driven validation

The private checker records `Some` of the original bytes for a present field
and `None` for an absent optional field. The shared accumulating traversal
preserves those values in schema order. Only this specialization converts
invalid issues to `Diagnostic`; missing required fields still produce an issue.

```ken
fn env_config_validation
      (checked : SchemaValidation EnvConfigOrigin (Option Bytes))
    : Validation (NonEmpty Diagnostic) (List (Option Bytes)) =
  match checked {
    Valid values ↦ Valid (NonEmpty Diagnostic) (List (Option Bytes)) values;
    Invalid issues ↦
      Invalid
        (NonEmpty Diagnostic)
        (List (Option Bytes))
        (nonempty_map
          (SchemaIssue EnvConfigOrigin)
          Diagnostic
          env_config_issue_diagnostic
          issues)
  }

fn decode_environment_entries
      (schema : Schema) (entries : List (Prod Bytes Bytes))
    : Validation (NonEmpty Diagnostic) (List (Option Bytes)) =
  env_config_validation
    (schema_validate EnvConfigOrigin (Option Bytes) (environment_field_check entries) schema)

export decode_process_environment, decode_config_entries, env_config_help
```

## 4. Laws and proofs

The required-field laws instantiate the schema traversal's checked coverage.
They connect each accepted required field to `env_config_lookup` and to the
aligned value in the very same validation payload. The lookup's `Some` value
is the raw entry value, with no text conversion or re-encoding. The optional
absence endpoint is no longer an empty-byte placeholder: an absent optional
field is carried as `None` and remains distinguishable from `Some` of empty
bytes. The two former optional-placeholder proofs and their second-traversal
helpers have no surviving claim.

```ken
pub proof required_lookup for decode_process_environment
      (schema : Schema)
      (input : ProcessInput)
      (values : List (Option Bytes))
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (decode_process_environment schema input)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values))
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
        (Option (Option Bytes))
        (Data.Collections.Derived.nth (Option Bytes) i values)
        (Some (Option Bytes) (Some Bytes value))
        → goal)
    : goal =
  let checked : SchemaValidation EnvConfigOrigin (Option Bytes) =
    schema_validate
      EnvConfigOrigin
      (Option Bytes)
      (environment_field_check (process_environment input))
      schema
  in
    env_config_schema_validation_cases
      checked
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
            (env_config_decode_valid_values checked checked_values values hchecked hdecode)
            (environment_required_check_lookup (process_environment input))
            i
            field
            hfield
            hpresence
            goal
            recover)
      (λissues.
        λhchecked.
          absurd (env_config_decode_invalid_impossible checked issues values hchecked hdecode))

pub proof required_lookup for decode_config_entries
      (schema : Schema)
    : (entries : List (Prod Bytes Bytes))
      → (values : List (Option Bytes))
      → Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (decode_config_entries schema entries)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values)
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
          (Option (Option Bytes))
          (Data.Collections.Derived.nth (Option Bytes) i values)
          (Some (Option Bytes) (Some Bytes value))
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
                    let checked : SchemaValidation EnvConfigOrigin (Option Bytes) =
                      schema_validate
                        EnvConfigOrigin
                        (Option Bytes)
                        (config_field_check entries)
                        schema
                    in
                      env_config_schema_validation_cases
                        checked
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
                                checked
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
                                checked
                                issues
                                values
                                hchecked
                                hdecode))

theorem env_config_valid_injective
      (e : Type)
      (a : Type)
      (left : a)
      (right : a)
      (same : Equal (Validation e a) (Valid e a left) (Valid e a right))
    : Equal a left right =
  same

theorem env_config_accepted_injective
      (origin : Type)
      (value : Type)
      (left : value)
      (right : value)
      (same : Equal
        (SchemaFieldCheck origin value)
        (Application.Input.Schema.SchemaFieldAccepted origin value left)
        (Application.Input.Schema.SchemaFieldAccepted origin value right))
    : Equal value left right =
  same

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
      (accepted : Option Bytes)
      (haccepted : Equal
        (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
        (env_config_field_check
          origin
          entries
          (Application.Input.Schema.MkSchemaField name presence shape documentation))
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin (Option Bytes) accepted))
      (goal : Prop)
      (recover : (value : Bytes)
        → Equal
        (Option Bytes)
        (env_config_lookup (bytes_encode name) entries)
        (Some Bytes value)
        → Equal
        (Option Bytes)
        accepted
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
              (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
              (match choice {
                Some value ↦
                  schema_field_accept EnvConfigOrigin (Option Bytes) (Some Bytes value);
                None ↦
                  env_config_missing_field
                    origin
                    (Application.Input.Schema.MkSchemaField
                      name
                      Application.Input.Schema.SchemaRequired
                      shape
                      documentation)
              })
              (Application.Input.Schema.SchemaFieldAccepted
                EnvConfigOrigin
                (Option Bytes)
                accepted))
          (J
            (λpresence2 _.
              Equal
                (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
                (env_config_field_check
                  origin
                  entries
                  (Application.Input.Schema.MkSchemaField name presence2 shape documentation))
                (Application.Input.Schema.SchemaFieldAccepted
                  EnvConfigOrigin
                  (Option Bytes)
                  accepted))
            haccepted
            hpresence)
          hlookup))
    (λvalue.
      λhlookup.
        recover
          value
          hlookup
          (env_config_accepted_injective
            EnvConfigOrigin
            (Option Bytes)
            accepted
            (Some Bytes value)
            (sym
              (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
              (Application.Input.Schema.SchemaFieldAccepted
                EnvConfigOrigin
                (Option Bytes)
                (Some Bytes value))
              (Application.Input.Schema.SchemaFieldAccepted
                EnvConfigOrigin
                (Option Bytes)
                accepted)
              (J
                (λchoice _.
                  Equal
                    (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
                    (match choice {
                      Some found ↦
                        schema_field_accept EnvConfigOrigin (Option Bytes) (Some Bytes found);
                      None ↦
                        env_config_missing_field
                          origin
                          (Application.Input.Schema.MkSchemaField
                            name
                            presence
                            shape
                            documentation)
                    })
                    (Application.Input.Schema.SchemaFieldAccepted
                      EnvConfigOrigin
                      (Option Bytes)
                      accepted))
                haccepted
                hlookup))))

theorem environment_required_check_lookup
      (entries : List (Prod Bytes Bytes)) (field : SchemaField)
    : (accepted : Option Bytes)
      → Equal
        (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
        (environment_field_check entries field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin (Option Bytes) accepted)
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
          accepted
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
    : (accepted : Option Bytes)
      → Equal
        (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
        (config_field_check entries field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin (Option Bytes) accepted)
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
          accepted
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
      (inspect : SchemaField → SchemaFieldCheck EnvConfigOrigin (Option Bytes))
      (fields : List SchemaField)
      (entries : List (Prod Bytes Bytes))
      (checked_values : List (Option Bytes))
      (values : List (Option Bytes))
      (hvalid : Equal
        (SchemaValidation EnvConfigOrigin (Option Bytes))
        (Application.Input.Schema.schema_validate_fields
          EnvConfigOrigin
          (Option Bytes)
          inspect
          fields)
        (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) checked_values))
      (hvalues : Equal (List (Option Bytes)) checked_values values)
      (accepted_lookup : (field : SchemaField)
        → (accepted : Option Bytes)
        → Equal
        (SchemaFieldCheck EnvConfigOrigin (Option Bytes))
        (inspect field)
        (Application.Input.Schema.SchemaFieldAccepted EnvConfigOrigin (Option Bytes) accepted)
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
          → Equal
          (Option Bytes)
          accepted
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
          (Option (Option Bytes))
          (Data.Collections.Derived.nth (Option Bytes) i values)
          (Some (Option Bytes) (Some Bytes value))
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
                (Option Bytes)
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
                            λhaccepted.
                              recover
                                value
                                hlookup
                                (J
                                  (λvalues2 _.
                                    Equal
                                      (Option (Option Bytes))
                                      (Data.Collections.Derived.nth (Option Bytes) i values2)
                                      (Some (Option Bytes) (Some Bytes value)))
                                  (trans
                                    (Option (Option Bytes))
                                    (Data.Collections.Derived.nth
                                      (Option Bytes)
                                      i
                                      checked_values)
                                    (Some (Option Bytes) accepted)
                                    (Some (Option Bytes) (Some Bytes value))
                                    hnth
                                    (cong
                                      (Option Bytes)
                                      (Option (Option Bytes))
                                      accepted
                                      (Some Bytes value)
                                      (Some (Option Bytes))
                                      haccepted))
                                  hvalues)))

theorem env_config_schema_validation_cases
      (checked : SchemaValidation EnvConfigOrigin (Option Bytes))
    : (goal : Prop)
      → ((values : List (Option Bytes))
          → Equal
          (SchemaValidation EnvConfigOrigin (Option Bytes))
          checked
          (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) values)
          → goal)
      → ((issues : NonEmpty (SchemaIssue EnvConfigOrigin))
          → Equal
          (SchemaValidation EnvConfigOrigin (Option Bytes))
          checked
          (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) issues)
          → goal)
      → goal =
  match checked {
    Valid values ↦ λgoal. λrecover_valid. λrecover_invalid. recover_valid values Refl;
    Invalid issues ↦ λgoal. λrecover_valid. λrecover_invalid. recover_invalid issues Refl
  }

theorem env_config_decode_invalid_impossible
      (checked : SchemaValidation EnvConfigOrigin (Option Bytes))
      (issues : NonEmpty (SchemaIssue EnvConfigOrigin))
      (values : List (Option Bytes))
      (hchecked : Equal
        (SchemaValidation EnvConfigOrigin (Option Bytes))
        checked
        (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) issues))
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (env_config_validation checked)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values))
    : Bottom =
  J
    (λchecked2 _.
      Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (env_config_validation checked2)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values)
      → Bottom)
    (λhdecode2. absurd hdecode2)
    (sym
      (SchemaValidation EnvConfigOrigin (Option Bytes))
      checked
      (Invalid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) issues)
      hchecked)
    hdecode

theorem env_config_decode_valid_values
      (checked : SchemaValidation EnvConfigOrigin (Option Bytes))
      (checked_values : List (Option Bytes))
      (values : List (Option Bytes))
      (hchecked : Equal
        (SchemaValidation EnvConfigOrigin (Option Bytes))
        checked
        (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) checked_values))
      (hdecode : Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (env_config_validation checked)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values))
    : Equal (List (Option Bytes)) checked_values values =
  J
    (λchecked2 _.
      Equal
        (Validation (NonEmpty Diagnostic) (List (Option Bytes)))
        (env_config_validation checked2)
        (Valid (NonEmpty Diagnostic) (List (Option Bytes)) values)
      → Equal (List (Option Bytes)) checked_values values)
    (λhdecode2.
      env_config_valid_injective
        (NonEmpty Diagnostic)
        (List (Option Bytes))
        checked_values
        values
        hdecode2)
    (sym
      (SchemaValidation EnvConfigOrigin (Option Bytes))
      checked
      (Valid (NonEmpty (SchemaIssue EnvConfigOrigin)) (List (Option Bytes)) checked_values)
      hchecked)
    hdecode
```

## 5. Trust and boundaries

The public surface consists of `decode_process_environment`,
`decode_config_entries`, `env_config_help`, and `env_config_lookup`, together
with one attached required-field law on each decoder. The lookup's existing
first-match behavior is the single authority used by validation, and its three
implementation helpers remain private.

The decoder consumes `process_environment`, shared `Schema`, `Validation`,
`Diagnostic`, the landed lawful `DecEq Bytes`, and Transport's checked `sym`
`cong`, and `trans`. It adds no parser, renderer,
location carrier, cached length, primitive, postulate, `Axiom`, or trusted-base
entry. Raw values—including invalid UTF-8—are returned without a `String` hop.
