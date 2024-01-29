use crate::*;

pub trait SemaExprDb {
    // todo: move this somewhere else
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion;
}

impl SemaExprDb for ::salsa::Db {
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion {
        sema_expr_region(self, syn_expr_region)
    }
}

#[salsa::jar]
pub struct SemaExprJar(
    ty_ontology_path_unveil_impl_block_signature_templates,
    ty_ontology_application_unveil_impl_block_signature_templates,
    SemaExprRegion,
    sema_expr_region,
    crate::helpers::analysis::sema_expr_region_contains_gn,
    crate::helpers::range::SemaExprRangeRegion,
    crate::helpers::range::sema_expr_range_region,
);
