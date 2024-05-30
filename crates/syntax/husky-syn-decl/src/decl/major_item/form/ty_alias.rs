use super::*;

#[salsa::tracked]
pub struct TypeAliasSynNodeDecl {
    #[id]
    pub syn_node_path: FormSynNodePath,
    #[return_ref]
    pub template_parameter_obelisk_list:
        SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAliasSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        chain_as_ref_err_collect!(self.template_parameter_obelisk_list(db), self.eq_token(db),)
    }
}

#[salsa::tracked]
pub struct TypeAliasSynDecl {
    #[id]
    pub path: MajorFormPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub ty_term: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAliasSynDecl {
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: TypeAliasSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_obelisk_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(
            db,
            path,
            template_parameters,
            expr,
            syn_expr_region,
        ))
    }
}
