use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct PropsStructTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub fields: SmallVec<[PropsStructFieldDeclarativeSignatureTemplate; 4]>,
    pub instance_constructor_ritchie_ty: DeclarativeTermRitchie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct PropsStructFieldDeclarativeSignatureTemplate {
    ident: Ident,
    ty: DeclarativeTerm,
    has_initialization: bool,
}

impl PropsStructTypeDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: PropsStructTypeSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        let fields = decl
            .fields(db)
            .iter()
            .enumerate()
            .map(|(i, field)| {
                Ok(PropsStructFieldDeclarativeSignatureTemplate {
                    ident: field.ident(),
                    ty: match declarative_term_region.expr_term(field.ty_syn_expr_idx()) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(DeclarativeSignatureError::FieldTypeDeclarativeTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    },
                    has_initialization: field.initialization().is_some(),
                })
            })
            .collect::<DeclarativeSignatureResult<SmallVec<_>>>()?;
        let instance_constructor_ritchie_ty =
            DeclarativeTermRitchie::new(db, RitchieKind::RITCHIE_TYPE_FN, fields
                .iter()
                .copied()
                .filter_map(
                    PropsStructFieldDeclarativeSignatureTemplate::into_ritchie_parameter_contracted_ty,
                )
                .collect(), self_ty);
        Ok(Self::new(
            db,
            template_parameters,
            self_ty,
            fields,
            instance_constructor_ritchie_ty,
        ))
    }
}

impl PropsStructFieldDeclarativeSignatureTemplate {
    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }

    pub fn into_ritchie_parameter_contracted_ty(self) -> Option<DeclarativeRitchieParameter> {
        (!self.has_initialization)
            .then_some(DeclarativeRitchieRegularParameter::new(TermContract::Move, self.ty).into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct PropsStructDeclarativeSignature {}
