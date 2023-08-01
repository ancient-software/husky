use husky_entity_syn_tree::ImplBlockSynNode;

use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeMethodFnDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_parameter: DeclarativeTermRitchieRegularParameter,
    /// parenate is a word I coined
    ///
    /// it means things that should be parenthesized
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_method_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeMethodFnSynDecl,
) -> DeclarativeSignatureResult<TraitForTypeMethodFnDeclarativeSignatureTemplate> {
    let self_parameter = DeclarativeTermRitchieRegularParameter::new(
        match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::None,
        },
        decl.path(db)
            .impl_block(db)
            .declarative_signature_template(db)?
            .self_ty(db)
            .term(),
    );
    let syn_expr_region = decl.syn_expr_region(db);
    let expr_region_data = syn_expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
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
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TraitForTypeMethodFnDeclarativeSignatureTemplate::new(
        db,
        template_parameters,
        self_parameter,
        parenate_parameters,
        return_ty,
    ))
}
