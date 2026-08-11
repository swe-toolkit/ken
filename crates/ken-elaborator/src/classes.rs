//! Typeclass environment — registry for class/instance declarations (`33 §5`).
//!
//! This module is a **pure data layer**: no kernel calls, no elaboration.
//! All elaboration logic lives in `elab.rs` which has access to the private
//! `ElabCtx` type. This module exports only the data structures that `elab.rs`
//! populates and `ElabEnv` carries.

use crate::ast::DefKeyword;
use crate::error::Span;
use crate::resolve::RType;
use ken_kernel::{GlobalId, Term};
use std::collections::HashMap;

/// Whether a class is a property class (Ω-sorted Σ-chain, coherence-free via
/// Ω-PI) or a structure class (Type-sorted, canonical-one-per-head rule).
/// Determined at declaration time by the kernel's `sort_sigma` (`check.rs:192`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassKind {
    /// All fields are Ω-sorted — the class is a proposition.
    /// Multiple instances for the same head-type are fine (Ω-PI: all are
    /// definitionally equal), so the overlap check is skipped.
    Property,
    /// At least one field is Type-sorted — the class carries computational
    /// content. Exactly one canonical instance per `(class, head-type)` key is
    /// enforced by the overlap check.
    Structure,
}

/// Per-class metadata registered at declaration time.
pub struct ClassInfo {
    /// The optional single type-parameter name (e.g. `A` in `class Eq A`).
    pub param: Option<String>,
    /// Elaborated class-parameter kind. Absent means the class is nullary.
    pub param_kind: Option<Term>,
    /// Field names in declaration order.
    pub field_names: Vec<String>,
    /// Field types in declaration order — a real Σ-telescope (`33 §5.2`):
    /// `field_types[i]` is a kernel `Term` valid in context
    /// `[a?, field_types[0], …, field_types[i-1]]` (the class param, if
    /// any, then every EARLIER field's type, outermost/highest-index
    /// first). Used to compute each instance field's properly-substituted
    /// expected type (`ken_kernel::subst::subst_tel`) and to drive `.field`
    /// projection (`elab.rs`'s `RExpr::RProj`).
    pub field_types: Vec<Term>,
    /// Optional SURF-2 purity marker per field. This metadata is erased before
    /// kernel admission; it must stay parallel to `field_names`/`field_types`.
    pub field_purities: Vec<Option<DefKeyword>>,
    /// Kernel `GlobalId` of the class's Σ-record type (`C : Type → sort`).
    pub type_id: GlobalId,
    /// Whether this is a property or structure class (`33 §5.1`).
    pub kind: ClassKind,
    /// Module where this class was declared (for orphan check, `33 §5.3`).
    pub module_id: u32,
}

#[derive(Clone, Copy)]
pub struct ProjectionView<'a> {
    pub owner_name: &'a str,
    pub type_id: GlobalId,
    pub head_param: Option<&'a str>,
    pub field_names: &'a [String],
    pub field_types: &'a [Term],
}

#[derive(Clone, Copy)]
pub struct ClassView<'a> {
    pub projection: ProjectionView<'a>,
    pub param_kind: Option<&'a Term>,
    pub field_purities: &'a [Option<DefKeyword>],
    pub kind: &'a ClassKind,
    pub module_id: u32,
}

impl ClassInfo {
    fn into_named_field(self) -> NamedFieldInfo {
        NamedFieldInfo {
            projection: ProjectionInfo {
                type_id: self.type_id,
                head_param: self.param,
                field_names: self.field_names,
                field_types: self.field_types,
            },
            kind: NamedFieldKind::Class(ClassOnlyInfo {
                param_kind: self.param_kind,
                field_purities: self.field_purities,
                kind: self.kind,
                module_id: self.module_id,
            }),
        }
    }
}

/// Projection metadata shared by every named-field owner.
struct ProjectionInfo {
    type_id: GlobalId,
    head_param: Option<String>,
    field_names: Vec<String>,
    field_types: Vec<Term>,
}

impl ProjectionInfo {
    fn view<'a>(&'a self, owner_name: &'a str) -> ProjectionView<'a> {
        ProjectionView {
            owner_name,
            type_id: self.type_id,
            head_param: self.head_param.as_deref(),
            field_names: &self.field_names,
            field_types: &self.field_types,
        }
    }
}

/// Metadata meaningful only for class owners and instance resolution.
struct ClassOnlyInfo {
    param_kind: Option<Term>,
    field_purities: Vec<Option<DefKeyword>>,
    kind: ClassKind,
    module_id: u32,
}

impl ClassOnlyInfo {
    fn view<'a>(&'a self, projection: ProjectionView<'a>) -> ClassView<'a> {
        ClassView {
            projection,
            param_kind: self.param_kind.as_ref(),
            field_purities: &self.field_purities,
            kind: &self.kind,
            module_id: self.module_id,
        }
    }
}

/// The closed owner classification stored atomically with projection facts.
enum NamedFieldKind {
    Class(ClassOnlyInfo),
    Record,
}

/// The sole private registry entry for every named-field owner.
struct NamedFieldInfo {
    projection: ProjectionInfo,
    kind: NamedFieldKind,
}

/// Per-instance metadata.
#[derive(Clone)]
pub struct InstanceInfo {
    /// Kernel `GlobalId` of the instance's Σ-record value.
    pub instance_id: GlobalId,
    /// Class this instance inhabits. Used only by surface projection purity.
    pub class_name: String,
    /// Inferred effect row for each instance field, in class-field order.
    pub field_effect_rows: Vec<crate::effects::RowType>,
    /// Module where this instance was declared (for orphan check).
    pub module_id: u32,
    /// Type parameters abstracted by the instance head, in source order.
    pub head_param_count: usize,
    /// The resolved instance-head pattern.  It selects which concrete type
    /// arguments are applied at a use site (e.g. `Pair a Bool` abstracts only
    /// `a`, not the fixed `Bool` slot).
    pub head_type: Option<RType>,
    /// Prerequisite dictionaries, retained so use-site resolution can build
    /// their recursive applications rather than merely returning the head id.
    pub constraints: Vec<InstanceConstraintInfo>,
    /// Dotted source-package path that defined this instance (N4 provenance).
    /// Direct, non-loader elaboration uses the stable `<local>` sentinel.
    pub defining_package: String,
    /// Original declaration span, retained so overlap reports both sites.
    pub declaration_span: Span,
}

/// Structured provenance emitted by a successful implicit resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceResolution {
    pub instance_id: GlobalId,
    pub class_name: String,
    pub head_type: String,
    pub defining_package: String,
}

/// A prerequisite dictionary required by a polymorphic instance.
#[derive(Clone)]
pub struct InstanceConstraintInfo {
    pub class_name: String,
    /// Surface-resolved shape, used to select the recursively required head.
    pub head_type: RType,
    /// Kernel type in the instance-head parameter context, used to close the
    /// instance Pi/lambda telescope and to kernel-check applications.
    pub core_type: Term,
}

/// The typeclass environment: class registry, canonical instance registry,
/// structural postulate IDs, and per-module tracking for the orphan check.
pub struct ClassEnv {
    named_field_owners: HashMap<String, NamedFieldInfo>,
    /// Canonical instances: `(class_name, head_type_name)` → `InstanceInfo`.
    /// For property classes this may hold multiple under different keys, but
    /// only one per `(class, head)` pair (property instances on the same head
    /// are accepted by Ω-PI, so no duplicate-key registration occurs — each
    /// instance is still a distinct value; the property check just waives the
    /// overlap error at the *second* registration).
    pub instances: HashMap<(String, String), InstanceInfo>,
    /// `RecordNil : Omega 0` — the Σ-chain prop terminator.
    pub record_nil_id: GlobalId,
    /// `record_nil_val : RecordNil` — the unique inhabitant.
    pub record_nil_val_id: GlobalId,
    /// Current module counter (bumped at module boundaries).
    pub current_module: u32,
    /// Maps each `GlobalId` to the module where it was declared.
    pub global_modules: HashMap<GlobalId, u32>,
    /// Package of the source unit currently elaborating through the N2 loader.
    pub current_package: Option<String>,
    /// Explicit direct-use roots of the active `program`/`package` boundary.
    /// `None` keeps direct, non-loader elaboration backward compatible.
    pub direct_use_packages: Option<std::collections::HashSet<String>>,
    /// Canonical dictionaries granted through an admitted package's
    /// re-exported public class/head surface (`33 §5.5.1`). This is kept
    /// separate from package admission so coherence closure never turns into
    /// general transitive dispatch.
    pub direct_use_instances: std::collections::HashSet<GlobalId>,
    /// Whether a boundary-less source closure may use its sole provider
    /// package without an explicit admission declaration.
    pub implicit_single_provider: bool,
    /// Distinct source packages which registered instances in this closure.
    pub source_instance_packages: std::collections::HashSet<String>,
    /// Successful implicit-resolution provenance in source order.
    pub resolution_provenance: Vec<InstanceResolution>,
}

impl ClassEnv {
    /// Enumerate every registered class through the storage-independent
    /// borrowed view.
    pub fn class_entries(&self) -> impl Iterator<Item = ClassView<'_>> + '_ {
        self.named_field_owners
            .iter()
            .filter_map(|(owner_name, info)| match &info.kind {
                NamedFieldKind::Class(class_info) => {
                    Some(class_info.view(info.projection.view(owner_name)))
                }
                NamedFieldKind::Record => None,
            })
    }

    pub fn class(&self, name: &str) -> Option<ClassView<'_>> {
        let (owner_name, info) = self.named_field_owners.get_key_value(name)?;
        match &info.kind {
            NamedFieldKind::Class(class_info) => {
                Some(class_info.view(info.projection.view(owner_name)))
            }
            NamedFieldKind::Record => None,
        }
    }

    pub fn projection_by_type_id(&self, id: GlobalId) -> Option<ProjectionView<'_>> {
        self.named_field_owners
            .iter()
            .find_map(|(owner_name, info)| {
                (info.projection.type_id == id).then(|| info.projection.view(owner_name))
            })
    }

    pub fn register_class(&mut self, name: String, info: ClassInfo) {
        self.named_field_owners
            .insert(name, info.into_named_field());
    }

    pub(crate) fn register_record(
        &mut self,
        name: String,
        type_id: GlobalId,
        field_names: Vec<String>,
        field_types: Vec<Term>,
    ) {
        self.named_field_owners.insert(
            name,
            NamedFieldInfo {
                projection: ProjectionInfo {
                    type_id,
                    head_param: None,
                    field_names,
                    field_types,
                },
                kind: NamedFieldKind::Record,
            },
        );
    }
    pub fn initialized(record_nil_id: GlobalId, record_nil_val_id: GlobalId) -> Self {
        Self {
            named_field_owners: HashMap::new(),
            instances: HashMap::new(),
            record_nil_id,
            record_nil_val_id,
            current_module: 0,
            global_modules: HashMap::new(),
            current_package: None,
            direct_use_packages: None,
            direct_use_instances: std::collections::HashSet::new(),
            implicit_single_provider: false,
            source_instance_packages: std::collections::HashSet::new(),
            resolution_provenance: Vec::new(),
        }
    }
    /// Create a sentinel `ClassEnv` for non-class elaboration paths. Its
    /// `GlobalId(0)` aliases a real declaration, so `elaborate_rdecl` rejects
    /// every class-dependent form before this environment is constructed. The
    /// real `ClassEnv` is created by `elab::init_class_env`, which pre-declares
    /// the structural postulates in the kernel.
    pub fn sentinel() -> Self {
        ClassEnv {
            named_field_owners: HashMap::new(),
            instances: HashMap::new(),
            record_nil_id: GlobalId(0),
            record_nil_val_id: GlobalId(0),
            current_module: 0,
            global_modules: HashMap::new(),
            current_package: None,
            direct_use_packages: None,
            direct_use_instances: std::collections::HashSet::new(),
            implicit_single_provider: false,
            source_instance_packages: std::collections::HashSet::new(),
            resolution_provenance: Vec::new(),
        }
    }

    /// Advance the module counter (call at module boundaries for orphan check).
    pub fn next_module(&mut self) {
        self.current_module += 1;
    }

    /// Look up the canonical instance for `(class_name, head_type_name)`.
    pub fn instance_search(&self, class_name: &str, head_name: &str) -> Option<GlobalId> {
        self.instances
            .get(&(class_name.to_string(), head_name.to_string()))
            .map(|i| i.instance_id)
    }
}
