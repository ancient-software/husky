use super::*;
use husky_entity_taxonomy::{EntityKind, FormKind, ModuleItemKind};
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
            Token::Keyword(keyword) => match keyword {
                Keyword::Connection(keyword) => match keyword {
                    ConnectionKeyword::For | ConnectionKeyword::Where => {
                        return TokenResolveResult::Break(())
                    }
                },
                Keyword::Pronoun(pronoun) => match pronoun {
                    PronounKeyword::Crate => todo!(),
                    PronounKeyword::SelfType => match self.allow_self_ty() {
                        AllowSelfType::True => ResolvedToken::AtomicExpr(Expr::SelfType(token_idx)),
                        AllowSelfType::False => ResolvedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::SelfTypeNotAllowed(token_idx).into(),
                        )),
                    },
                    PronounKeyword::SelfValue => match self.peek() {
                        Some(Token::Punctuation(Punctuation::Binary(
                            BinaryOpr::ScopeResolution,
                        ))) => {
                            todo!()
                        }
                        _ => match self.allow_self_value() {
                            AllowSelfValue::True => {
                                ResolvedToken::AtomicExpr(Expr::SelfValue(token_idx))
                            }
                            AllowSelfValue::False => ResolvedToken::AtomicExpr(Expr::Err(
                                OriginalExprError::SelfValueNotAllowed(token_idx).into(),
                            )),
                        },
                    },

                    PronounKeyword::Super => todo!(),
                },
                _ => ResolvedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedKeyword(token_idx).into(),
                )),
            },
            Token::Ident(ident) => self.resolve_ident(token_idx, ident),
            Token::Label(_) => todo!(),
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
                Punctuation::LaOrLt => match self.top_expr() {
                    TopExprRef::Unfinished(_) => todo!(),
                    TopExprRef::Finished(expr) => {
                        match expr.base_entity_path(self.db(), &self.parser.expr_arena) {
                            BaseEntityPath::Uncertain {
                                inclination: BaseEntityPathInclination::TypeOrVariant,
                            } => ResolvedToken::Bra(token_idx, Bracket::Angle),
                            BaseEntityPath::Some(entity_path) => {
                                match entity_path.entity_kind(self.db()) {
                                    EntityKind::Module => todo!(),
                                    EntityKind::ModuleItem {
                                        module_item_kind,
                                        connection,
                                    } => match module_item_kind {
                                        ModuleItemKind::Form(FormKind::Value) => {
                                            ResolvedToken::BinaryOpr(
                                                token_idx,
                                                BinaryComparisonOpr::Less.into(),
                                            )
                                        }
                                        ModuleItemKind::Type(_)
                                        | ModuleItemKind::Form(_)
                                        | ModuleItemKind::Trait => {
                                            ResolvedToken::Bra(token_idx, Bracket::Angle)
                                        }
                                    },
                                    EntityKind::AssociatedItem {
                                        associated_item_kind,
                                    } => todo!(),
                                    EntityKind::Variant => todo!(),
                                }
                            }
                            _ => ResolvedToken::BinaryOpr(
                                token_idx,
                                BinaryOpr::Comparison(BinaryComparisonOpr::Less),
                            ),
                        }
                    }
                    TopExprRef::None => todo!(),
                },
                Punctuation::ColonColonLAngle => todo!(),
                Punctuation::RaOrGt => match (self.last_bra(), self.env_bra()) {
                    (Some(Bracket::Angle), _) => ResolvedToken::Ket(token_idx, Bracket::Angle),
                    (None, Some(Bracket::Angle)) => return TokenResolveResult::Break(()),
                    _ => ResolvedToken::BinaryOpr(token_idx, BinaryComparisonOpr::Greater.into()),
                },
                Punctuation::Sheba => ResolvedToken::AtomicExpr(Expr::Err(
                    OriginalExprError::UnexpectedSheba(token_idx).into(),
                )),
                Punctuation::Shr => {
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Shift(BinaryShiftOpr::Shr))
                }
                Punctuation::DeriveAssign => return TokenResolveResult::Break(()),
                Punctuation::Minus => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Minus),
                Punctuation::Exclamation => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Not),
                Punctuation::DoubleVertical => todo!(),
                Punctuation::Tilde => ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Tilde),
                Punctuation::Dot => ResolvedToken::Dot(token_idx),
                Punctuation::Colon => match self.last_unfinished_expr() {
                    Some(UnfinishedExpr::List {
                        opr: UnfinishedListOpr::BoxList { .. },
                        items,
                        ..
                    }) => {
                        if items.len() == 0 && self.finished_expr().is_none() {
                            ResolvedToken::ColonRightAfterLBox(token_idx)
                        } else {
                            match self.token_stream.is_empty() {
                                true => return TokenResolveResult::Break(()),
                                false => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                            }
                        }
                    }
                    _ => match self.peek() {
                        // not end of token group
                        Some(_) => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Ins),
                        // end of token group
                        None => return TokenResolveResult::Break(()),
                    },
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
                            BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                        ),
                        false => ResolvedToken::Bra(token_idx, Bracket::Vertical),
                    },
                },
                Punctuation::DoubleExclamation => todo!(),
                Punctuation::Semicolon => return TokenResolveResult::Break(()),
                Punctuation::XmlKet => return TokenResolveResult::Break(()),
                Punctuation::At => return TokenResolveResult::Break(()),
                Punctuation::AtEq => return TokenResolveResult::Break(()),
                Punctuation::Question => match self.finished_expr() {
                    // only see `?` as Option when there obviously is no other way
                    Some(Expr::List { .. }) | Some(Expr::BoxColonList { .. }) | None => {
                        ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Option)
                    }
                    Some(expr) => {
                        ResolvedToken::SuffixOpr(token_idx, SuffixOpr::UnveilOrComposeWithOption)
                    }
                },
                Punctuation::PoundSign => return TokenResolveResult::Break(()),
                Punctuation::Ambersand => match self.finished_expr() {
                    Some(Expr::List { .. }) | None => {
                        ResolvedToken::PrefixOpr(token_idx, PrefixOpr::Ref)
                    }
                    Some(_) => ResolvedToken::BinaryOpr(
                        token_idx,
                        BinaryOpr::Closed(BinaryClosedOpr::BitOr),
                    ),
                },
                Punctuation::DotDot => todo!(),
                Punctuation::ColonColon => {
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::ScopeResolution)
                }
                Punctuation::Star => {
                    ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Closed(BinaryClosedOpr::Mul))
                }
                Punctuation::Eq => match self.env() {
                    Some(env) => match env {
                        ExprEnvironment::LetVariablesType => match self.last_bra() {
                            Some(_) => todo!(),
                            None => return TokenResolveResult::Break(()),
                        },
                        ExprEnvironment::WithinBracket(_) => todo!(),
                    },
                    None => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::Assign),
                },
                Punctuation::EqEq => todo!(),
                Punctuation::ForAll => todo!(),
                Punctuation::Exists => todo!(),
            },
            Token::WordOpr(opr) => match opr {
                WordOpr::And => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::And),
                ),
                WordOpr::Or => ResolvedToken::BinaryOpr(
                    token_idx,
                    BinaryOpr::ShortCircuitLogic(BinaryShortcuitLogicOpr::Or),
                ),
                WordOpr::As => ResolvedToken::BinaryOpr(token_idx, BinaryOpr::As),
                WordOpr::Be => ResolvedToken::Be(token_idx),
            },
            Token::Literal(_) => ResolvedToken::AtomicExpr(Expr::Literal(token_idx)),
            Token::Error(error) => {
                ResolvedToken::AtomicExpr(Expr::Err(DerivedExprError::Token(error).into()))
            }
        })
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn resolve_ident(&mut self, token_idx: TokenIdx, ident: Ident) -> ResolvedToken {
        if let Some(opn) = self.last_unfinished_expr() {
            match opn {
                UnfinishedExpr::Binary {
                    punctuation: BinaryOpr::ScopeResolution,
                    lopd,
                    ..
                } => match lopd.base_entity_path(self.db(), &self.parser.expr_arena) {
                    BaseEntityPath::None => todo!(),
                    BaseEntityPath::Some(_) => {
                        p!(
                            lopd,
                            ident.debug(self.parser.db),
                            self.parser.path.debug(self.parser.db)
                        );
                        todo!()
                    }
                    BaseEntityPath::Uncertain { .. } => {
                        return ResolvedToken::AtomicExpr(Expr::Err(
                            OriginalExprError::UnresolvedSubentity { token_idx, ident }.into(),
                        ))
                    }
                    BaseEntityPath::Err => todo!(),
                },
                _ => (),
            }
        }
        ResolvedToken::AtomicExpr(
            match self
                .parser
                .symbol_context
                .resolve_ident(self.db(), token_idx, ident)
            {
                Some(symbol) => match symbol {
                    // Symbol::Variable(variable_idx) => Expr::Variable {
                    //     token_idx,
                    //     symbol_idx: variable_idx,
                    // },
                    Symbol::Entity(entity_path) => {
                        let (entity_path_expr, entity_path) =
                            self.parse_entity_path_expr(token_idx, ident, entity_path);
                        Expr::EntityPath {
                            entity_path_expr,
                            path: entity_path,
                        }
                    }
                    Symbol::Inherited(inherited_symbol_idx, inherited_symbol_kind) => {
                        Expr::InheritedSymbol {
                            ident,
                            token_idx,
                            inherited_symbol_idx,
                            inherited_symbol_kind,
                        }
                    }
                    Symbol::Local(current_symbol_idx, current_symbol_kind) => Expr::CurrentSymbol {
                        ident,
                        token_idx,
                        current_symbol_idx,
                        current_symbol_kind,
                    },
                    //  Expr::EntityPath(entity_path),
                },
                None => Expr::Err(OriginalExprError::UnrecognizedIdent { token_idx, ident }.into()),
            },
        )
    }
}

#[derive(Debug)]
pub(crate) enum ResolvedToken {
    AtomicExpr(Expr),
    BinaryOpr(TokenIdx, BinaryOpr),
    PrefixOpr(TokenIdx, PrefixOpr),
    SuffixOpr(TokenIdx, SuffixOpr),
    Bra(TokenIdx, Bracket),
    Ket(TokenIdx, Bracket),
    Dot(TokenIdx),
    ListItem(TokenIdx),
    Be(TokenIdx),
    ColonRightAfterLBox(TokenIdx),
}
