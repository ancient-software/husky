mod call_list;
mod comma_list;

pub(super) use self::call_list::*;
pub(super) use self::comma_list::*;

use super::*;
use husky_token_data::delimiter::Delimiter;
use parsec::TryParseOptionFromStream;
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum IncompleteSynExprData {
    Binary {
        lopd: SynExprData,
        punctuation: SynBinaryOpr,
        punctuation_regional_token_idx: RegionalTokenIdx,
    },
    Prefix {
        punctuation: SynPrefixOpr,
        punctuation_regional_token_idx: RegionalTokenIdx,
    },
    /// list separated by commas
    /// ```husky
    /// A(a, b, c)
    /// ```
    CommaList {
        opr: IncompleteCommaListOpr,
        // todo: move this into opr
        bra: Delimiter,
        // todo: move this into opr
        bra_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
    },
    /// call list includes more separators like `;`
    CallList {
        opr: IncompleteCallListOpr,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCallListItem; 4]>,
    },
    LambdaHead {
        // todo: use SmallVec
        // inputs: Vec<(RangedIdent, Option<SynExprIdx>)>,
        // start: TextPosition,
    },
    Application {
        function: SynExprData,
    },
    /// just needs the return type
    Ritchie {
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparRegionalToken,
        argument_tys: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
        light_arrow_token: LightArrowRegionalToken,
    },
    KeyedArgument {
        key_regional_token_idx: RegionalTokenIdx,
        key: Ident,
        eq_token: EqRegionalToken,
    },
}

impl<'a, C> TryParseOptionFromStream<SynExprParser<'a, C>> for SynHtmlArgumentExpr
where
    C: IsSynExprContext<'a>,
{
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynExprParser<'a, C>,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(lcurl) = sp.try_parse_option::<InlineLcurlRegionalToken>()? {
            Ok(Some(SynHtmlArgumentExpr::Shortened {
                lcurl,
                property_ident: sp.try_parse_expected(OriginalSynExprError::HtmlTodo)?,
                rcurl: sp.try_parse_expected(OriginalSynExprError::HtmlTodo)?,
            }))
        } else if let Some(argument_ident) = sp.try_parse_option::<IdentRegionalToken>()? {
            Ok(Some(SynHtmlArgumentExpr::Expanded {
                property_ident: argument_ident,
                eq: sp.try_parse_expected(OriginalSynExprError::HtmlTodo)?,
                lcurl: sp.try_parse_expected(OriginalSynExprError::HtmlTodo)?,
                expr: sp.parse_expr_expected2(
                    None,
                    SynExprRootKind::HtmlArgumentExpr,
                    OriginalSynExprError::HtmlTodo,
                ),
                rcurl: sp.try_parse_expected(OriginalSynExprError::HtmlTodo)?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl IncompleteSynExprData {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            IncompleteSynExprData::Binary { punctuation, .. } => punctuation.precedence(),
            IncompleteSynExprData::Prefix { .. } => Precedence::Prefix,
            IncompleteSynExprData::CommaList { .. } | IncompleteSynExprData::CallList { .. } => {
                Precedence::List
            }
            IncompleteSynExprData::LambdaHead { .. } => Precedence::LambdaHead,
            IncompleteSynExprData::Application { .. } => Precedence::Application,
            IncompleteSynExprData::Ritchie { .. } => Precedence::Curry,
            IncompleteSynExprData::KeyedArgument { .. } => Precedence::KeyedArgument,
        }
    }
}
