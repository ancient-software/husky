mod call;
mod feature;
mod global;
mod memb;
mod traits;
mod ty;

pub use call::*;
pub use memb::*;
pub use traits::*;
pub use ty::*;

use ast::*;
use entity_syntax::RawTyKind;
use feature::*;
use file::FilePtr;
use fold::FoldStorage;
use global::*;
use infer_error::*;
use entity_route::*;
use scope_query::*;
use std::sync::Arc;
use vm::Compiled;
use word::CustomIdentifier;

#[salsa::query_group(InferSignatureQueryGroupStorage)]
pub trait InferSignatureQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: EntityRoutePtr) -> InferResultArc<CallSignature>;
    fn ty_signature(&self, scope: EntityRoutePtr) -> InferResultArc<TySignature>;
    fn feature_signature(&self, scope: EntityRoutePtr) -> InferResultArc<FeatureSignature>;
    fn global_input_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn vec_signature_template(&self) -> Arc<TySignature>;
}
