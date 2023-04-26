#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(trait_upcasting)]
mod data;
mod db;
mod disambiguation;
mod engine;
mod error;
mod expectation;
mod nested;
mod progress;
mod region;
mod resolve;
mod signature;
mod substitution;
mod term;
#[cfg(test)]
mod tests;
mod utils;

pub use self::data::*;
pub use self::db::*;
pub use self::disambiguation::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::expectation::*;
pub use self::progress::*;
pub use self::region::*;
pub use self::resolve::*;
pub use self::substitution::*;
pub use self::term::*;

pub(crate) use self::signature::*;
#[cfg(test)]
pub(crate) use self::tests::*;

use self::nested::*;
use either::*;
use husky_declarative_signature::*;
use husky_entity_path::*;
use husky_ethereal_term::*;
use husky_expr::*;
use husky_print_utils::p;
use husky_term_prelude::*;
use maybe_result::*;
use salsa::DebugWithDb as _;
use smallvec::*;

#[salsa::jar(db = FluffyTermDb)]
pub struct FluffyTermJar(
    term_ritchie_fluffy_data,
    term_application_fluffy_data,
    ethereal_ty_ontology_path_ty_field_disambiguation,
    ethereal_term_application_ty_field_disambiguation,
    ethereal_ty_ontology_path_ty_method_disambiguation,
    ethereal_term_application_ty_method_disambiguation,
);
