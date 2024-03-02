use super::*;
use either::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db]
pub struct DeclarativeParenateParameters {
    data: SmallVec<[DeclarativeRitchieParameter; 4]>,
}

impl std::ops::Deref for DeclarativeParenateParameters {
    type Target = [DeclarativeRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DeclarativeParenateParameters {
    pub(crate) fn from_decl(
        parameters: &[ParenateParameterSyndicate],
        syn_expr_region_data: &SynExprRegionData,
        signature_region: &SynExprDecTermRegion,
    ) -> DecSignatureResult<Self> {
        Ok(Self {
            data: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    Ok(match parameter {
                        ParenateParameterSyndicate::Simple {
                            syn_pattern_root,
                            variables,
                            colon,
                            ty,
                        } => DeclarativeRitchieSimpleParameter::new(
                            syn_expr_region_data
                                .pattern_contract(syn_pattern_root.syn_pattern_expr_idx()),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DecSignatureError::ParameterTypeDecTermError(i.try_into().unwrap())
                            })?,
                        )
                        .into(),
                        ParenateParameterSyndicate::Variadic {
                            symbol_modifier_keyword_group,
                            ty,
                            ..
                        } => DeclarativeRitchieVariadicParameter::new(
                            Contract::new(*symbol_modifier_keyword_group),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DecSignatureError::ParameterTypeDecTermError(i.try_into().unwrap())
                            })?,
                        )
                        .into(),
                        ParenateParameterSyndicate::Keyed {
                            symbol_modifier_keyword_group,
                            ident_token,
                            ty,
                            default,
                            ..
                        } => DeclarativeRitchieKeyedParameter::new(
                            ident_token.ident(),
                            Contract::new(*symbol_modifier_keyword_group),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DecSignatureError::ParameterTypeDecTermError(i.try_into().unwrap())
                            })?,
                            match *default {
                                Left(_) => false,
                                Right(_) => true,
                                // Some(
                                //     signature_region.expr_term(default_expr_idx).map_err(|_| {
                                //         DecSignatureError::ParameterTypeDecTermError(
                                //             i.try_into().unwrap(),
                                //         )
                                //     })?,
                                // ),
                            },
                        )
                        .into(),
                    })
                })
                .collect::<DecSignatureResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[DeclarativeRitchieParameter] {
        &self.data
    }
}
