use super::*;
use husky_print_utils::p;
use husky_token::Punctuation;
use salsa::DebugWithDb;
use std::ops::ControlFlow;

pub type TokenResolveResult<T> = ControlFlow<(), T>;

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn resolve_token(
        &mut self,
        token_idx: TokenIdx,
        token: Token,
    ) -> TokenResolveResult<ResolvedToken> {
        TokenResolveResult::Continue(match token {
            Token::Attr(_) => todo!(),
            Token::Keyword(keyword) => {
                ResolvedToken::Atom(Expr::Err(ExprError::UnexpectedKeyword(token_idx)))
            }
            Token::Identifier(ident) => self.resolve_ident(token_idx, ident),
            Token::Punctuation(punc) => match punc {
                Punctuation::Binary(binary) => ResolvedToken::BinaryOpr(token_idx, binary),
                Punctuation::Bra(bra) => ResolvedToken::Bra(token_idx, bra),
                Punctuation::Ket(ket) => match self.last_bra() {
                    Some(bra) => {
                        if bra != ket {
                            p!(bra, ket);
                            p!(self.unfinished_exprs());
                            todo!()
                        }
                        ResolvedToken::Ket(token_idx, ket)
                    }
                    None => return TokenResolveResult::Break(()),
                },
                Punctuation::Suffix(suffix) => ResolvedToken::SuffixOpr(token_idx, suffix),
                Punctuation::LAngle => match self.top_expr() {
                    TopExprRef::Unfinished(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::None => ResolvedToken::BinaryOpr(
                                token_idx,
                                BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                            BaseEntityPath::Some(_) => todo!(),
                            BaseEntityPath::Uncertain { inclination } => match inclination {
                                BaseEntityPathInclination::GlobalValue
                                | BaseEntityPathInclination::TypeOrVariant => {
                                    ResolvedToken::Bra(token_idx, Bracket::Angle)
                                }
                                BaseEntityPathInclination::FunctionOrLocalValue => {
                                    ResolvedToken::BinaryOpr(
                                        token_idx,
                                        BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                                    )
                                }
                            },
                        }
                    }
                    TopExprRef::None => todo!(),
                },
                Punctuation::RAngle => match (self.last_bra(), self.env()) {
                    (Some(Bracket::Angle), _) => ResolvedToken::Ket(token_idx, Bracket::Angle),
                    (None, ExprParseEnvironment::WithinBracket(Bracket::Angle)) => {
                        return TokenResolveResult::Break(())
                    }
                    _ => ResolvedToken::BinaryOpr(token_idx, BinaryComparisonOpr::Greater.into()),
                },
                Punctuation::DeriveAssign => return TokenResolveResult::Break(()),
                Punctuation::Minus => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Minus),
                Punctuation::Exclamation => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Not),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::BitNot => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::BitNot),
                Punctuation::Dot => ResolvedToken::Dot(token_idx),
                Punctuation::Colon => match self.peek() {
                    Some(_) => todo!(),
                    None => return TokenResolveResult::Break(()),
                },
                Punctuation::Comma => {
                    self.reduce(Precedence::ListItem);
                    match self.last_unfinished_expr() {
                        Some(expr) => match expr {
                            UnfinishedExpr::List { .. } => ResolvedToken::ListItem(token_idx),
                            _ => return TokenResolveResult::Break(()),
                        },
                        None => return TokenResolveResult::Break(()),
                    }
                }
                Punctuation::Vertical => match self.last_unfinished_expr() {
                    Some(UnfinishedExpr::List {
                        bra: Bracket::Vertical,
                        ..
                    }) => ResolvedToken::Ket(token_idx, Bracket::Vertical),
                    _ => match self.finished_expr().is_some() {
                        true => ResolvedToken::BinaryOpr(
                            token_idx,
                            BinaryOpr::PureClosed(BinaryPureClosedOpr::BitOr),
                        ),
                        false => ResolvedToken::Bra(token_idx, Bracket::Vertical),
                    },
                },
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => return TokenResolveResult::Break(()),
                Punctuation::XmlKet => todo!(),
                Punctuation::At => todo!(),
                Punctuation::Question => match self.finished_expr() {
                    Some(_) => ResolvedToken::SuffixOpr(token_idx, SuffixOpr::Unveil),
                    None => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Option),
                },
                Punctuation::PoundSign => todo!(),
                Punctuation::Ambersand => match self.finished_expr() {
                    Some(_) => ResolvedToken::BinaryOpr(
                        token_idx,
                        BinaryOpr::PureClosed(BinaryPureClosedOpr::BitOr),
                    ),
                    None => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Ref),
                },
                Punctuation::DotDot => todo!(),
            },
            Token::WordOpr(opr) => match opr {
                WordOpr::And => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortcuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortcuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::As),
                WordOpr::Be => ResolvedToken::Be(token_idx),
            },
            Token::Literal(_) => ResolvedToken::Atom(Expr::Literal(token_idx)),
            Token::Err(ref error) => ResolvedToken::Atom(Expr::Err(error.clone().into())),
        })
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    lopd,
                    ..
                } => match lopd.base_entity_path(self.db(), &self.parser.expr_arena) {
                    BaseEntityPath::None => todo!(),
                    BaseEntityPath::Some(_) => todo!(),
                    BaseEntityPath::Uncertain { .. } => {
                        return ResolvedToken::Atom(Expr::Uncertain(ident))
                    }
                },
                _ => (),
            }
        }
        ResolvedToken::Atom(
            match self.parser.symbol_context.resolve_ident(token_idx, ident) {
                Some(symbol) => match symbol {
                    // Symbol::Variable(variable_idx) => Expr::Variable {
                    //     token_idx,
                    //     symbol_idx: variable_idx,
                    // },
                    Symbol::Entity(entity_path) => todo!(),
                    Symbol::Inherited(inherited_symbol_idx) => Expr::InheritedSymbol {
                        ident,
                        token_idx,
                        inherited_symbol_idx,
                    },
                    Symbol::Local(local_symbol_idx, local_symbol_kind) => Expr::LocalSymbol {
                        ident,
                        token_idx,
                        local_symbol_idx,
                        local_symbol_kind,
                    },
                    //  Expr::EntityPath(entity_path),
                },
                None => Expr::Unrecognized(ident),
            },
        )
    }
}

#[derive(Debug)]
pub(crate) enum ResolvedToken {
    Atom(Expr),
    BinaryOpr(TokenIdx, BinaryOpr),
    PrefixOpr(TokenIdx, PrefixOpr),
    SuffixOpr(TokenIdx, SuffixOpr),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    ListItem(TokenIdx),
    Be(TokenIdx),
}
