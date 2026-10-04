//! Compiler-independent run-time support for linked Ken executables.
//!
//! The linked executable uses this crate's single static archive; the
//! compiler depends on this crate for the same boundary contracts.

pub mod activation_abi;
pub mod activation_services;
pub mod artifact_validation;
pub mod boundary_activation;
pub mod boundary_resource_profile;
pub mod boundary_value;
pub mod canonical;
pub mod hash;
#[doc(hidden)]
pub mod invocation_tickets;
pub mod ir;
pub mod native_int;
pub mod store;
pub mod values;

pub use activation_abi::*;
pub use activation_services::*;
pub use artifact_validation::*;
pub use boundary_activation::*;
pub use boundary_resource_profile::*;
pub use canonical::Canonical;
pub use hash::fnv1a_64;
pub use ir::*;
pub use native_int::*;
pub use store::{InternResult, Space, Store, StoreStats};
pub use values::{Sign, Value};
