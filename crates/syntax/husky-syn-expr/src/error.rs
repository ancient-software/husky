use crate::*;
use husky_token_data::delimiter::Delimiter;
use original_error::OriginalError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum SynExprError {
    #[error("original {0}")]
    Original(#[from] OriginalSynExprError),
    #[error("derived {0}")]
    Derived(#[from] DerivedSynExprError),
}

impl From<TokenDataError> for SynExprError {
    fn from(value: TokenDataError) -> Self {
        SynExprError::Derived(value.into())
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum OriginalSynExprError {
    #[error("expected `>`")]
    ExpectedRightAngleDelimiter {
        langle_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected `}}`")]
    ExpectedRightCurlyBrace(RegionalTokenStreamState),
    #[error("expected identifier")]
    ExpectedIdent(RegionalTokenStreamState),
    #[error("expected `:`")]
    ExpectedColon(RegionalTokenStreamState),
    #[error("expected `)`")]
    ExpectedRpar(RegionalTokenStreamState),
    #[error("no matching bracket")]
    NoMatchingBra {
        ket: Delimiter,
        ket_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected item before `,`")]
    ExpectedItemBeforeComma {
        comma_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected item before `be`")]
    ExpectedItemBeforeBe {
        be_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected variable pattern")]
    ExpectedLetPattern(RegionalTokenStreamState),
    #[error("expected pattern expression after `be`")]
    ExpectedBePattern(RegionalTokenStreamState),
    #[error("expected pattern expression after `|`")]
    ExpectedCasePattern(RegionalTokenStreamState),
    #[error("expected `=`")]
    ExpectedAssign(RegionalTokenStreamState),
    #[error("expected initial value")]
    ExpectedInitialValue(RegionalTokenStreamState),
    #[error("unexpected keyword")]
    UnexpectedKeyword(RegionalTokenIdx),
    #[error("expected result")]
    ExpectedResult(RegionalTokenStreamState),
    #[error("expected condition")]
    ExpectedCondition(RegionalTokenStreamState),
    #[error("expected match expression")]
    ExpectedMatchExpr(RegionalTokenStreamState),
    #[error("expected end of line `with`")]
    ExpectedEolWithInMatchHead(RegionalTokenStreamState),
    #[error("expected for expr")]
    ExpectedForExpr(RegionalTokenIdx),
    #[error("expected paramter pattern")]
    ExpectedParameterPattern(RegionalTokenIdx),
    #[error("ExpectedValueForFieldBindInitialization")]
    ExpectedValueForFieldBindInitialization(RegionalTokenStreamState),
    #[error("ExpectedFunctionIdentAfterOpeningHtmlBra")]
    ExpectedFunctionIdentAfterOpeningHtmlBra(RegionalTokenStreamState),
    #[error("expected identifier after modifier")]
    ExpectedIdentAfterModifier(RegionalTokenStreamState, EphemSymbolModifierRegionalTokens),
    #[error("expected `:` at end of line")]
    ExpectedEolColon(RegionalTokenStreamState),
    #[error("expected constant implicit parameter type")]
    ExpectedConstantImplicitParameterType(RegionalTokenStreamState),
    #[error("mismatching bracket")]
    MismatchingDelimiter {
        bra: Delimiter,
        bra_regional_token_idx: RegionalTokenIdx,
        ket: Delimiter,
        ket_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected let variables type")]
    ExpectedLetVariablesType(RegionalTokenStreamState),
    #[error("expected field type")]
    ExpectedFieldType(RegionalTokenStreamState),
    #[error("expected parameter type")]
    ExpectedParameterType(RegionalTokenStreamState),
    #[error("ExpectedTraits")]
    ExpectedTraits(RegionalTokenStreamState),
    #[error("ExpectedKeyedWithDefaultParameterInitialValue")]
    ExpectedExplicitParameterDefaultValue(RegionalTokenStreamState),
    #[error("expected `=>` after case pattern")]
    ExpectedHeavyArrowAfterCasePattern(RegionalTokenStreamState),
    #[error("expected identifier after `::`")]
    ExpectIdentAfterScopeResolution(RegionalTokenStreamState),
    #[error("expected identifier after `.`")]
    ExpectedIdentAfterDot {
        dot_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expected exprBeforeDot")]
    ExpectedExprBeforeDot {
        dot_regional_token_idx: RegionalTokenIdx,
    },
    #[error("expect block")]
    ExpectedBlock(RegionalTokenVerseIdx),
    #[error("unterminated list")]
    UnterminatedList {
        bra_regional_token_idx: RegionalTokenIdx,
    },
    #[error("unterminated list")]
    UnterminatedFunctionCallKeyedArgumentList {
        bra_regional_token_idx: RegionalTokenIdx,
    },
    #[error("unterminated list")]
    UnterminatedMethodCallKeyedArgumentList {
        bra_regional_token_idx: RegionalTokenIdx,
    },
    #[error("no left operand for binary operator")]
    NoLeftOperandForBinaryOperator {
        binary_regional_token_idx: RegionalTokenIdx,
    },
    #[error("no right operand for binary operator")]
    NoRightOperandForBinaryOperator {
        punctuation: SynBinaryOpr,
        punctuation_regional_token_idx: RegionalTokenIdx,
    },
    #[error("no operand for prefix operator")]
    NoOperandForPrefixOperator {
        prefix: SynPrefixOpr,
        prefix_regional_token_idx: RegionalTokenIdx,
    },
    #[error("unexpected `$`")]
    UnexpectedSheba(RegionalTokenIdx),
    #[error("unrecognized identifier")]
    UnrecognizedIdent {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
    },
    #[error("unrecognized identifier")]
    UnresolvedSubitem {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
    },
    #[error("SelfTypeNotAllowed")]
    SelfTypeNotAllowed(RegionalTokenIdx),
    #[error("SelfValueNotAllowed")]
    SelfValueNotAllowed(RegionalTokenIdx),
    #[error("HtmlTodo")]
    HtmlTodo(RegionalTokenStreamState),
    #[error("UnexpectedLeftCurlyBrace")]
    UnexpectedInlineLcurl(RegionalTokenIdx),
    #[error("ExpectedTypeAfterLightArrow")]
    ExpectedTypeAfterLightArrow {
        light_arrow_token: LightArrowRegionalToken,
    },
    #[error("ExpectedTypeTermForAssocType")]
    ExpectedTypeTermForAssocType(RegionalTokenStreamState),
    #[error("entity tree error")]
    EntityTree {
        regional_token_idx: RegionalTokenIdx,
        error: EntityTreeError,
    },
    #[error("ExpectedBlockRcurl")]
    ExpectedBlockRcurl(RegionalTokenStreamState),
    #[error("ExpectedRvertForClosure")]
    ExpectedRvertForClosure(RegionalTokenStreamState),
    #[error("ExpectedEqTokenAfterReturnTypeForClosure")]
    ExpectedEqTokenAfterReturnTypeForClosure(RegionalTokenStreamState),
    #[error("ExpectedReturnTypeAfterLightArrowForClosure")]
    ExpectedReturnTypeAfterLightArrowForClosure(RegionalTokenStreamState),
    #[error("ExpectedBodyExprForClosure")]
    ExpectedBodyExprForClosure(RegionalTokenStreamState),
}

impl OriginalSynExprError {
    pub fn regional_token_idx_range(&self) -> RegionalTokenIdxRange {
        match self {
            OriginalSynExprError::ExpectedLetPattern(token_stream_state)
            | OriginalSynExprError::ExpectedBePattern(token_stream_state)
            | OriginalSynExprError::ExpectedCasePattern(token_stream_state)
            | OriginalSynExprError::ExpectedLetVariablesType(token_stream_state)
            | OriginalSynExprError::ExpectedAssign(token_stream_state)
            | OriginalSynExprError::ExpectedInitialValue(token_stream_state)
            | OriginalSynExprError::ExpectedResult(token_stream_state)
            | OriginalSynExprError::ExpectedCondition(token_stream_state)
            | OriginalSynExprError::ExpectedMatchExpr(token_stream_state)
            | OriginalSynExprError::ExpectedEolWithInMatchHead(token_stream_state)
            | OriginalSynExprError::ExpectedRightCurlyBrace(token_stream_state)
            | OriginalSynExprError::ExpectedIdent(token_stream_state)
            | OriginalSynExprError::ExpectedColon(token_stream_state)
            | OriginalSynExprError::ExpectedRpar(token_stream_state)
            | OriginalSynExprError::ExpectedEolColon(token_stream_state)
            | OriginalSynExprError::ExpectedIdentAfterModifier(token_stream_state, _)
            | OriginalSynExprError::ExpectedFieldType(token_stream_state)
            | OriginalSynExprError::ExpectedParameterType(token_stream_state)
            | OriginalSynExprError::HtmlTodo(token_stream_state)
            | OriginalSynExprError::ExpectedValueForFieldBindInitialization(token_stream_state)
            | OriginalSynExprError::ExpectedFunctionIdentAfterOpeningHtmlBra(token_stream_state)
            | OriginalSynExprError::ExpectedConstantImplicitParameterType(token_stream_state)
            | OriginalSynExprError::ExpectedTraits(token_stream_state)
            | OriginalSynExprError::ExpectedExplicitParameterDefaultValue(token_stream_state)
            | OriginalSynExprError::ExpectedHeavyArrowAfterCasePattern(token_stream_state) => {
                let regional_token_idx = token_stream_state.next_regional_token_idx();
                match token_stream_state.drained() {
                    true => RegionalTokenIdxRange::new_drained(regional_token_idx),
                    false => RegionalTokenIdxRange::new_single(regional_token_idx),
                }
            }
            OriginalSynExprError::MismatchingDelimiter {
                ket_regional_token_idx: regional_token_idx,
                ..
            }
            | OriginalSynExprError::ExpectedRightAngleDelimiter {
                langle_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::NoMatchingBra {
                ket_regional_token_idx: regional_token_idx,
                ..
            }
            | OriginalSynExprError::NoLeftOperandForBinaryOperator {
                binary_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::NoRightOperandForBinaryOperator {
                punctuation_regional_token_idx: regional_token_idx,
                ..
            }
            | OriginalSynExprError::NoOperandForPrefixOperator {
                prefix_regional_token_idx: regional_token_idx,
                ..
            }
            | OriginalSynExprError::UnexpectedKeyword(regional_token_idx)
            | OriginalSynExprError::ExpectedItemBeforeComma {
                comma_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::ExpectedItemBeforeBe {
                be_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::ExpectedForExpr(regional_token_idx)
            | OriginalSynExprError::ExpectedParameterPattern(regional_token_idx)
            | OriginalSynExprError::UnterminatedList {
                bra_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::UnterminatedFunctionCallKeyedArgumentList {
                bra_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::UnterminatedMethodCallKeyedArgumentList {
                bra_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::UnexpectedSheba(regional_token_idx)
            | OriginalSynExprError::UnrecognizedIdent {
                regional_token_idx, ..
            }
            | OriginalSynExprError::UnresolvedSubitem {
                regional_token_idx, ..
            }
            | OriginalSynExprError::SelfTypeNotAllowed(regional_token_idx)
            | OriginalSynExprError::SelfValueNotAllowed(regional_token_idx)
            | OriginalSynExprError::ExpectedIdentAfterDot {
                dot_regional_token_idx: regional_token_idx,
                ..
            }
            | OriginalSynExprError::ExpectedExprBeforeDot {
                dot_regional_token_idx: regional_token_idx,
            }
            | OriginalSynExprError::UnexpectedInlineLcurl(regional_token_idx) => {
                RegionalTokenIdxRange::new_single(*regional_token_idx)
            }
            OriginalSynExprError::ExpectedBlock(_) => todo!(),
            OriginalSynExprError::ExpectedTypeAfterLightArrow { .. } => todo!(),
            OriginalSynExprError::ExpectedTypeTermForAssocType(_) => todo!(),
            OriginalSynExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
            OriginalSynExprError::EntityTree { .. } => todo!(),
            OriginalSynExprError::ExpectedBlockRcurl(_) => todo!(),
            OriginalSynExprError::ExpectedRvertForClosure(_) => todo!(),
            OriginalSynExprError::ExpectedEqTokenAfterReturnTypeForClosure(_) => todo!(),
            OriginalSynExprError::ExpectedReturnTypeAfterLightArrowForClosure(_) => todo!(),
            OriginalSynExprError::ExpectedBodyExprForClosure(_) => todo!(),
        }
    }
}

impl OriginalError for OriginalSynExprError {
    type Error = SynExprError;
}

#[derive(Error, Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum DerivedSynExprError {
    #[error("token error {0}")]
    TokenData(#[from] TokenDataError),
}

pub type SynExprResult<T> = Result<T, SynExprError>;
pub type SynExprResultRef<'a, T> = Result<T, &'a SynExprError>;

// impl<'a, 'b> FromAbsent<RegionalRcurlToken, ExprParseContext<'a, 'b>> for OriginalExprError {
//     fn new_absent_error(state: <ExprParseContext<'a, 'b> as HasParseState>::State) -> Self {
//         OriginalExprError::ExpectRightCurlyBrace(state)
//     }
// }

// // impl<'a, Context> FromAbsent<RegionalIdentToken, Context> for ExprError
// // where
// //     Context: TokenParseContext<'a>,
// //
// // {
// //     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
// //         ExprError::ExpectIdent(state)
// //     }
// // }

// impl<'a, Context> FromAbsent<RegionalColonToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectColon(state)
//     }
// }

// impl<'a, Context> FromAbsent<RparToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectRightParenthesis(state)
//     }
// }

// impl<'a, Context> FromAbsent<LetVariableDecls, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectLetVariablePattern(state)
//     }
// }

// impl<'a, Context> FromAbsent<AssignToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectAssignToken(state)
//     }
// }

// impl<'a, Context> FromAbsent<BeVariableDeclPattern, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectBePattern(state)
//     }
// }

// impl<'a, Context> FromAbsent<EolColonToken, Context> for OriginalExprError
// where
//     Context: TokenParseContext<'a>,
// {
//     fn new_absent_error(state: <Context as parsec::HasParseState>::State) -> Self {
//         OriginalExprError::ExpectEolColon(state)
//     }
// }
