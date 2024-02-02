use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeMethodFnDecTemplate {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockDecTemplate,
    /// the term for `Self`
    /// not necessarily equal to the type of `self` which might be wrapped in & or &mut etc.
    ///
    /// we don't use self_ty_arguments because it's not determined for declarative terms
    pub self_ty: DecTerm,
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_value_parameter: DeclarativeRitchieRegularParameter,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DecTerm,
}

impl TypeMethodFnDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeMethodFnSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let expr_region_data = syn_expr_region.data(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let impl_block = decl.impl_block_path(db).dec_template(db)?;
        let self_ty = impl_block.ty(db);
        let contract = match decl.self_value_parameter(db) {
            Some(self_value_parameter) => {
                TermContract::new(self_value_parameter.ephem_symbol_modifier_token_group())
            }
            None => TermContract::Pure,
        };
        let self_value_parameter = DeclarativeRitchieRegularParameter::new(contract, self_ty);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            expr_region_data,
            declarative_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => declarative_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => declarative_term_menu.unit(),
        };
        Ok(TypeMethodFnDecTemplate::new(
            db,
            path,
            impl_block,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}
