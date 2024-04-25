use super::*;
use parsec::{PunctuatedSmallList, TryParseFromStream};

#[salsa::tracked]
pub struct PropsStructSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    lcurl: SynNodeDeclResult<PropsStructLcurlRegionalToken>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<PropsStructRcurlRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl PropsStructSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.lcurl(db).as_ref().err())
                .chain(self.fields(db).as_ref().err().into_iter())
                .chain(self.rcurl(db).as_ref().err().into_iter()),
        )
    }
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructLcurlRegionalToken(InlineLcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsStructLcurlRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructRcurlRegionalToken(InlineRcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsStructRcurlRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedRcurlForStruct)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked]
pub struct PropsStructSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl PropsStructSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypePath,
        syn_node_decl: PropsStructSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let fields = SmallVec::from(syn_node_decl.fields(db).as_ref()?.elements());
        syn_node_decl.rcurl(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            syn_expr_region,
        ))
    }
}
