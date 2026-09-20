# Schema

`Schema` is the client-independent description shared by command-line and
environment/config decoders. It owns only field identity, presence,
value-shape, documentation metadata, and the two traversals both clients run.
Acquisition, provenance, validation policy, and diagnostic rendering remain in
the clients.

## 1. Description vocabulary

```ken
import Capability.Formatting.Doc (Doc, Text)

import Data.Collections.Derived (list_append, nth)

import Data.Collections.NonEmpty (NonEmpty, nonempty_append, nonempty_cons)

import Data.Sums.Validation (Invalid, Valid, Validation)

data SchemaPresence = SchemaRequired | SchemaOptional

data SchemaValueShape = SchemaFlag | SchemaBytes

data SchemaField = MkSchemaField String SchemaPresence SchemaValueShape String

data Schema = MkSchema String String (List SchemaField)

fn schema_field_name (field : SchemaField) : String =
  match field {
    MkSchemaField name presence shape documentation ↦ name
  }

fn schema_field_presence (field : SchemaField) : SchemaPresence =
  match field {
    MkSchemaField name presence shape documentation ↦ presence
  }

fn schema_field_shape (field : SchemaField) : SchemaValueShape =
  match field {
    MkSchemaField name presence shape documentation ↦ shape
  }

fn schema_field_documentation (field : SchemaField) : String =
  match field {
    MkSchemaField name presence shape documentation ↦ documentation
  }

fn schema_name (schema : Schema) : String =
  match schema {
    MkSchema name documentation fields ↦ name
  }

fn schema_documentation (schema : Schema) : String =
  match schema {
    MkSchema name documentation fields ↦ documentation
  }

fn schema_fields (schema : Schema) : List SchemaField =
  match schema {
    MkSchema name documentation fields ↦ fields
  }
```

## 2. Generic accumulating validation

The checker is supplied by each client, so source-specific policy and origin
types stay local. The traversal nevertheless owns the accumulation shape and
visits every field.

```ken
data SchemaIssue origin = MkSchemaIssue origin String

data SchemaFieldCheck origin value =
  SchemaFieldAccepted value
  | SchemaFieldRejected (SchemaIssue origin)

const SchemaValidation (origin : Type) (value : Type) : Type =
  Validation (NonEmpty (SchemaIssue origin)) (List value)

fn schema_field_accept
      (origin : Type) (value : Type) (accepted : value)
    : SchemaFieldCheck origin value =
  SchemaFieldAccepted origin value accepted

fn schema_field_reject
      (origin : Type) (value : Type) (issue : SchemaIssue origin)
    : SchemaFieldCheck origin value =
  SchemaFieldRejected origin value issue

fn schema_check_presence
      (origin : Type)
      (value : Type)
      (optional_value : value)
      (required_issue : SchemaIssue origin)
      (presence : SchemaPresence)
    : SchemaFieldCheck origin value =
  match presence {
    SchemaRequired ↦ schema_field_reject origin value required_issue;
    SchemaOptional ↦ schema_field_accept origin value optional_value
  }

fn schema_issue_origin (origin : Type) (issue : SchemaIssue origin) : origin =
  match issue {
    MkSchemaIssue at code ↦ at
  }

fn schema_issue_code (origin : Type) (issue : SchemaIssue origin) : String =
  match issue {
    MkSchemaIssue at code ↦ code
  }

fn schema_validation_cons
      (origin : Type)
      (value : Type)
      (head : SchemaFieldCheck origin value)
      (tail : SchemaValidation origin value)
    : SchemaValidation origin value =
  match head {
    SchemaFieldAccepted accepted ↦
      match tail {
        Valid rest ↦
          Valid (NonEmpty (SchemaIssue origin)) (List value) (Cons value accepted rest);
        Invalid issues ↦ Invalid (NonEmpty (SchemaIssue origin)) (List value) issues
      };
    SchemaFieldRejected issue ↦
      match tail {
        Valid rest ↦
          Invalid
            (NonEmpty (SchemaIssue origin))
            (List value)
            (nonempty_cons (SchemaIssue origin) issue (Nil (SchemaIssue origin)));
        Invalid issues ↦
          Invalid
            (NonEmpty (SchemaIssue origin))
            (List value)
            (nonempty_append
              (SchemaIssue origin)
              (nonempty_cons (SchemaIssue origin) issue (Nil (SchemaIssue origin)))
              issues)
      }
  }

pub fn schema_validate_fields
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (fields : List SchemaField)
    : SchemaValidation origin value =
  match fields {
    Nil ↦ Valid (NonEmpty (SchemaIssue origin)) (List value) (Nil value);
    Cons field rest ↦
      schema_validation_cons
        origin
        value
        (inspect field)
        (schema_validate_fields origin value inspect rest)
  }

theorem schema_some_injective
      (a : Type) (left : a) (right : a) (same : Equal (Option a) (Some a left) (Some a right))
    : Equal a left right =
  same

theorem schema_valid_injective
      (e : Type)
      (a : Type)
      (left : a)
      (right : a)
      (same : Equal (Validation e a) (Valid e a left) (Valid e a right))
    : Equal a left right =
  same

theorem schema_sym
      (a : Type) (left : a) (right : a) (same : Equal a left right)
    : Equal a right left =
  J (λactual _. Equal a actual left) Refl same

theorem schema_field_check_cases
      (origin : Type) (value : Type) (outcome : SchemaFieldCheck origin value)
    : (goal : Prop)
      → ((issue : SchemaIssue origin)
          → Equal
          (SchemaFieldCheck origin value)
          outcome
          (SchemaFieldRejected origin value issue)
          → goal)
      → ((accepted : value)
          → Equal
          (SchemaFieldCheck origin value)
          outcome
          (SchemaFieldAccepted origin value accepted)
          → goal)
      → goal =
  match outcome {
    SchemaFieldRejected issue ↦ λgoal. λon_rejected. λon_accepted. on_rejected issue Refl;
    SchemaFieldAccepted accepted ↦ λgoal. λon_rejected. λon_accepted. on_accepted accepted Refl
  }

theorem schema_validation_cases
      (e : Type) (a : Type) (outcome : Validation e a)
    : (goal : Prop)
      → ((issues : e) → Equal (Validation e a) outcome (Invalid e a issues) → goal)
      → ((values : a) → Equal (Validation e a) outcome (Valid e a values) → goal)
      → goal =
  match outcome {
    Invalid issues ↦ λgoal. λon_invalid. λon_valid. on_invalid issues Refl;
    Valid values ↦ λgoal. λon_invalid. λon_valid. on_valid values Refl
  }

theorem schema_validation_cons_transport
      (origin : Type)
      (value : Type)
      (actual_head : SchemaFieldCheck origin value)
      (known_head : SchemaFieldCheck origin value)
      (same_head : Equal (SchemaFieldCheck origin value) actual_head known_head)
      (actual_tail : SchemaValidation origin value)
      (known_tail : SchemaValidation origin value)
      (same_tail : Equal (SchemaValidation origin value) actual_tail known_tail)
      (outcome : SchemaValidation origin value)
      (same_result : Equal
        (SchemaValidation origin value)
        (schema_validation_cons origin value actual_head actual_tail)
        outcome)
    : Equal
        (SchemaValidation origin value)
        (schema_validation_cons origin value known_head known_tail)
        outcome =
  J
    (λhead _.
      Equal
        (SchemaValidation origin value)
        (schema_validation_cons origin value head known_tail)
        outcome)
    (J
      (λtail _.
        Equal
          (SchemaValidation origin value)
          (schema_validation_cons origin value actual_head tail)
          outcome)
      same_result
      same_tail)
    same_head

theorem schema_nth_cons_zero_transport
      (a : Type)
      (head : a)
      (tail : List a)
      (values : List a)
      (same : Equal (List a) (Cons a head tail) values)
    : Equal (Option a) (nth a Zero values) (Some a head) =
  J (λactual _. Equal (Option a) (nth a Zero actual) (Some a head)) Refl same

theorem schema_nth_cons_suc_transport
      (a : Type)
      (head : a)
      (tail : List a)
      (values : List a)
      (i : Nat)
      (found : a)
      (hfound : Equal (Option a) (nth a i tail) (Some a found))
      (same : Equal (List a) (Cons a head tail) values)
    : Equal (Option a) (nth a (Suc i) values) (Some a found) =
  J (λactual _. Equal (Option a) (nth a (Suc i) actual) (Some a found)) hfound same

theorem schema_inspect_transport
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (head : SchemaField)
      (field : SchemaField)
      (accepted : value)
      (hhead : Equal
        (SchemaFieldCheck origin value)
        (inspect head)
        (SchemaFieldAccepted origin value accepted))
      (hfield : Equal (Option SchemaField) (Some SchemaField head) (Some SchemaField field))
    : Equal
        (SchemaFieldCheck origin value)
        (inspect field)
        (SchemaFieldAccepted origin value accepted) =
  J
    (λactual _.
      Equal
        (SchemaFieldCheck origin value)
        (inspect actual)
        (SchemaFieldAccepted origin value accepted))
    hhead
    (schema_some_injective SchemaField head field hfield)

theorem schema_fields_valid_coverage_helper
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (fields : List SchemaField)
    : (values : List value)
      → Equal
        (SchemaValidation origin value)
        (schema_validate_fields origin value inspect fields)
        (Valid (NonEmpty (SchemaIssue origin)) (List value) values)
      → (goal : Prop)
      → (i : Nat)
      → (field : SchemaField)
      → Equal (Option SchemaField) (nth SchemaField i fields) (Some SchemaField field)
      → ((accepted : value)
          → Equal
          (Option value)
          (nth value i values)
          (Some value accepted)
          → Equal
          (SchemaFieldCheck origin value)
          (inspect field)
          (SchemaFieldAccepted origin value accepted)
          → goal)
      → goal =
  match fields {
    Nil ↦ λvalues. λhvalid. λgoal. λi. λfield. λhfield. λrecover. absurd hfield;
    Cons head tail ↦
      λvalues.
        λhvalid.
          λgoal.
            λi.
              match i {
                Zero ↦
                  λfield.
                    λhfield.
                      λrecover.
                        schema_field_check_cases
                          origin
                          value
                          (inspect head)
                          goal
                          (λissue.
                            λhhead.
                              schema_validation_cases
                                (NonEmpty (SchemaIssue origin))
                                (List value)
                                (schema_validate_fields origin value inspect tail)
                                goal
                                (λissues.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldRejected origin value issue)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Invalid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          issues)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid))
                                (λtail_values.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldRejected origin value issue)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          tail_values)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid)))
                          (λaccepted.
                            λhhead.
                              schema_validation_cases
                                (NonEmpty (SchemaIssue origin))
                                (List value)
                                (schema_validate_fields origin value inspect tail)
                                goal
                                (λissues.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldAccepted origin value accepted)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Invalid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          issues)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid))
                                (λtail_values.
                                  λhtail.
                                    recover
                                      accepted
                                      (schema_nth_cons_zero_transport
                                        value
                                        accepted
                                        tail_values
                                        values
                                        (schema_valid_injective
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          (Cons value accepted tail_values)
                                          values
                                          (schema_validation_cons_transport
                                            origin
                                            value
                                            (inspect head)
                                            (SchemaFieldAccepted origin value accepted)
                                            hhead
                                            (schema_validate_fields origin value inspect tail)
                                            (Valid
                                              (NonEmpty (SchemaIssue origin))
                                              (List value)
                                              tail_values)
                                            htail
                                            (Valid
                                              (NonEmpty (SchemaIssue origin))
                                              (List value)
                                              values)
                                            hvalid)))
                                      (schema_inspect_transport
                                        origin
                                        value
                                        inspect
                                        head
                                        field
                                        accepted
                                        hhead
                                        hfield)));
                Suc i2 ↦
                  λfield.
                    λhfield.
                      λrecover.
                        schema_field_check_cases
                          origin
                          value
                          (inspect head)
                          goal
                          (λissue.
                            λhhead.
                              schema_validation_cases
                                (NonEmpty (SchemaIssue origin))
                                (List value)
                                (schema_validate_fields origin value inspect tail)
                                goal
                                (λissues.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldRejected origin value issue)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Invalid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          issues)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid))
                                (λtail_values.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldRejected origin value issue)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          tail_values)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid)))
                          (λaccepted.
                            λhhead.
                              schema_validation_cases
                                (NonEmpty (SchemaIssue origin))
                                (List value)
                                (schema_validate_fields origin value inspect tail)
                                goal
                                (λissues.
                                  λhtail.
                                    absurd
                                      (schema_validation_cons_transport
                                        origin
                                        value
                                        (inspect head)
                                        (SchemaFieldAccepted origin value accepted)
                                        hhead
                                        (schema_validate_fields origin value inspect tail)
                                        (Invalid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          issues)
                                        htail
                                        (Valid
                                          (NonEmpty (SchemaIssue origin))
                                          (List value)
                                          values)
                                        hvalid))
                                (λtail_values.
                                  λhtail.
                                    schema_fields_valid_coverage_helper
                                      origin
                                      value
                                      inspect
                                      tail
                                      tail_values
                                      htail
                                      goal
                                      i2
                                      field
                                      hfield
                                      (λfound.
                                        λhfound.
                                          λhinspect.
                                            recover
                                              found
                                              (schema_nth_cons_suc_transport
                                                value
                                                accepted
                                                tail_values
                                                values
                                                i2
                                                found
                                                hfound
                                                (schema_valid_injective
                                                  (NonEmpty (SchemaIssue origin))
                                                  (List value)
                                                  (Cons value accepted tail_values)
                                                  values
                                                  (schema_validation_cons_transport
                                                    origin
                                                    value
                                                    (inspect head)
                                                    (SchemaFieldAccepted origin value accepted)
                                                    hhead
                                                    (schema_validate_fields
                                                      origin
                                                      value
                                                      inspect
                                                      tail)
                                                    (Valid
                                                      (NonEmpty (SchemaIssue origin))
                                                      (List value)
                                                      tail_values)
                                                    htail
                                                    (Valid
                                                      (NonEmpty (SchemaIssue origin))
                                                      (List value)
                                                      values)
                                                    hvalid)))
                                              hinspect)))
              }
  }

pub proof valid_coverage for schema_validate_fields
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (fields : List SchemaField)
      (values : List value)
      (hvalid : Equal
        (SchemaValidation origin value)
        (schema_validate_fields origin value inspect fields)
        (Valid (NonEmpty (SchemaIssue origin)) (List value) values))
    : (i : Nat)
      → (field : SchemaField)
      → Equal (Option SchemaField) (nth SchemaField i fields) (Some SchemaField field)
      → (goal : Prop)
      → ((accepted : value)
          → Equal
          (Option value)
          (nth value i values)
          (Some value accepted)
          → Equal
          (SchemaFieldCheck origin value)
          (inspect field)
          (SchemaFieldAccepted origin value accepted)
          → goal)
      → goal =
  λi.
    λfield.
      λhfield.
        λgoal.
          λrecover.
            schema_fields_valid_coverage_helper
              origin
              value
              inspect
              fields
              values
              hvalid
              goal
              i
              field
              hfield
              recover

pub proof accepted_tail_invalid for schema_validate_fields
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (field : SchemaField)
      (rest : List SchemaField)
      (accepted : value)
      (issues : NonEmpty (SchemaIssue origin))
      (hfield : Equal
        (SchemaFieldCheck origin value)
        (inspect field)
        (SchemaFieldAccepted origin value accepted))
      (htail : Equal
        (SchemaValidation origin value)
        (schema_validate_fields origin value inspect rest)
        (Invalid (NonEmpty (SchemaIssue origin)) (List value) issues))
    : Equal
        (SchemaValidation origin value)
        (schema_validate_fields origin value inspect (Cons SchemaField field rest))
        (Invalid (NonEmpty (SchemaIssue origin)) (List value) issues) =
  schema_validation_cons_transport
    origin
    value
    (SchemaFieldAccepted origin value accepted)
    (inspect field)
    (schema_sym
      (SchemaFieldCheck origin value)
      (inspect field)
      (SchemaFieldAccepted origin value accepted)
      hfield)
    (Invalid (NonEmpty (SchemaIssue origin)) (List value) issues)
    (schema_validate_fields origin value inspect rest)
    (schema_sym
      (SchemaValidation origin value)
      (schema_validate_fields origin value inspect rest)
      (Invalid (NonEmpty (SchemaIssue origin)) (List value) issues)
      htail)
    (Invalid (NonEmpty (SchemaIssue origin)) (List value) issues)
    Refl

fn schema_validate
      (origin : Type)
      (value : Type)
      (inspect : SchemaField → SchemaFieldCheck origin value)
      (schema : Schema)
    : SchemaValidation origin value =
  schema_validate_fields origin value inspect (schema_fields schema)
```

## 3. Shared help traversal

```ken
fn schema_presence_chars (presence : SchemaPresence) : List Char =
  match presence {
    SchemaRequired ↦ string_to_list_char "required";
    SchemaOptional ↦ string_to_list_char "optional"
  }

fn schema_shape_chars (shape : SchemaValueShape) : List Char =
  match shape {
    SchemaFlag ↦ Nil Char;
    SchemaBytes ↦ string_to_list_char " <value>"
  }

fn schema_field_label_chars (field : SchemaField) : List Char =
  list_append
    Char
    (string_to_list_char (schema_field_name field))
    (schema_shape_chars (schema_field_shape field))

fn schema_field_detail_chars (field : SchemaField) : List Char =
  list_append
    Char
    (string_to_list_char " [")
    (list_append
      Char
      (schema_presence_chars (schema_field_presence field))
      (string_to_list_char "]  "))

fn schema_field_help_chars (field : SchemaField) : List Char =
  list_append
    Char
    (string_to_list_char "  ")
    (list_append
      Char
      (schema_field_label_chars field)
      (list_append
        Char
        (schema_field_detail_chars field)
        (list_append
          Char
          (string_to_list_char (schema_field_documentation field))
          (string_to_list_char "\n"))))

fn schema_fields_help_chars (fields : List SchemaField) : List Char =
  match fields {
    Nil ↦ Nil Char;
    Cons field rest ↦
      list_append Char (schema_field_help_chars field) (schema_fields_help_chars rest)
  }

fn schema_help (schema : Schema) : Doc =
  Text
    (list_append
      Char
      (string_to_list_char (schema_name schema))
      (list_append
        Char
        (string_to_list_char " ")
        (list_append
          Char
          (string_to_list_char (schema_documentation schema))
          (list_append
            Char
            (string_to_list_char "\nFields:\n")
            (schema_fields_help_chars (schema_fields schema))))))

export SchemaPresence,
  SchemaRequired,
  SchemaOptional,
  SchemaValueShape,
  SchemaFlag,
  SchemaBytes,
  SchemaField,
  MkSchemaField,
  Schema,
  MkSchema,
  SchemaIssue,
  MkSchemaIssue,
  SchemaFieldCheck,
  SchemaFieldAccepted,
  SchemaFieldRejected,
  SchemaValidation,
  schema_field_name,
  schema_field_presence,
  schema_fields,
  schema_field_accept,
  schema_check_presence,
  schema_issue_origin,
  schema_issue_code,
  schema_validate_fields,
  schema_validate,
  schema_help
```

## 4. Trust and layering

`Schema` mentions neither client and adds no primitive, postulate, `Axiom`, or
trusted-base entry. Its result and issue carriers are parameterized over client
origin and value types.
