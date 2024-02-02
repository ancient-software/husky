use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeImplBlockDecTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub trai: DeclarativeTerm,
    pub self_ty: DeclarativeSelfType,
    // todo: where clause
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeSelfType {
    Path(DeclarativeTerm),
    DerivedAny(SymbolDeclarativeTerm),
}

impl DeclarativeSelfType {
    pub fn term(self) -> DeclarativeTerm {
        match self {
            DeclarativeSelfType::Path(term) => term,
            DeclarativeSelfType::DerivedAny(term) => term.into(),
        }
    }
}

impl HasDecTemplate for TraitForTypeImplBlockPath {
    type DecTemplate = TraitForTypeImplBlockDecTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        trai_for_ty_impl_block_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_impl_block_syn_declarative_signature_template(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> DeclarativeSignatureResult<TraitForTypeImplBlockDecTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = declarative_term_region.expr_term(trai_expr.syn_expr_idx())?;
    let self_ty_term = declarative_term_region
        .term_symbol_region()
        .self_ty()
        .ok_or(DeclarativeSignatureError::SelfTypeNotInferred)?;
    let self_ty = match decl.self_ty_decl(db) {
        SelfTypeDecl::PathLeadingExpr(ty_expr) => {
            debug_assert_eq!(
                self_ty_term,
                declarative_term_region
                    .expr_term(ty_expr.expr())
                    .expect("ok")
            );
            DeclarativeSelfType::Path(self_ty_term)
        }
        SelfTypeDecl::DeriveAny { .. } => {
            let DeclarativeTerm::Symbol(self_ty_symbol) = self_ty_term else {
                unreachable!()
            };
            DeclarativeSelfType::DerivedAny(self_ty_symbol)
        }
    };
    Ok(TraitForTypeImplBlockDecTemplate::new(
        db,
        template_parameters,
        trai,
        self_ty,
    ))
}
