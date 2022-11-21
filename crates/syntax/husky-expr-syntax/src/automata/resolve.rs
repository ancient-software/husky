use super::*;

use husky_symbol_syntax::Symbol;
use husky_term::TermItd;
use husky_token::SpecialToken;

impl<'a> Automata<'a> {
    pub(crate) fn resolve_token(&self, token: &Token) -> ResolvedToken {
        ResolvedToken {
            kind: self.resolve_token_kind(token),
            range: token.range,
        }
    }

    fn resolve_token_kind(&self, token: &Token) -> ResolvedTokenKind {
        match token.kind {
            TokenKind::Decorator(_) => todo!(),
            TokenKind::Keyword(_keyword) => todo!(),
            TokenKind::Identifier(ident) => self.resolve_ident(ident),
            TokenKind::Special(special) => match special {
                SpecialToken::BinaryOpr(opr) => ResolvedTokenKind::BinaryOpr(opr),
                SpecialToken::Bra(_) => todo!(),
                SpecialToken::Ket(_) => todo!(),
                SpecialToken::LAngle => todo!(),
                SpecialToken::RAngle => todo!(),
                SpecialToken::DeriveAssign => todo!(),
                SpecialToken::Minus => ResolvedTokenKind::Prefix(PrefixOpr::Minus),
                SpecialToken::Exclamation => ResolvedTokenKind::Prefix(PrefixOpr::Not),
                SpecialToken::Incr => ResolvedTokenKind::Suffix(RawSuffixOpr::Incr),
                SpecialToken::Decr => ResolvedTokenKind::Suffix(RawSuffixOpr::Decr),
                SpecialToken::DoubleVertical => todo!(),
                SpecialToken::BitNot => todo!(),
                SpecialToken::FieldAccess => todo!(),
                SpecialToken::BinaryOpr(BinaryOpr::Curry) => todo!(),
                SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution) => todo!(),
                SpecialToken::Colon => todo!(),
                SpecialToken::Comma => todo!(),
                SpecialToken::Ambersand => todo!(),
                SpecialToken::Vertical => todo!(),
                SpecialToken::DoubleExclamation => todo!(),
                SpecialToken::Semicolon => todo!(),
                SpecialToken::XmlKet => todo!(),
                SpecialToken::At => todo!(),
                SpecialToken::QuestionMark => todo!(),
            },
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::WordPattern(_) => todo!(),
            TokenKind::Literal(literal) => ResolvedTokenKind::Atom(literal.into()),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
        }
    }

    fn resolve_ident(&self, ident: Identifier) -> ResolvedTokenKind {
        if let Some(opr) = self.stack.top_opr() {
            match opr.variant {
                OnStackOprVariant::Binary(BinaryOpr::ScopeResolution) => {
                    if let Some(previous_expr) = self.stack.top_expr() {
                        match previous_expr.base_scope_result() {
                            BaseScopeResult::None => todo!(),
                            BaseScopeResult::Some(_) => todo!(),
                            BaseScopeResult::Uncertain => {
                                return ResolvedTokenKind::Atom(RawAtomExpr::Uncertain)
                            }
                        }
                    } else {
                        todo!()
                    }
                }
                _ => (),
            }
        }
        self.ctx.resolve_ident(ident).into()
    }

    fn resolve_previous_entity(&self) -> Option<TermItd> {
        self.stack.top_expr().map(|expr| self.resolve_entity(expr))
    }

    fn resolve_entity(&self, expr: &Expr) -> TermItd {
        match expr.variant {
            RawExprVariant::Atom(ref atom) => match atom {
                RawAtomExpr::Literal(_) => todo!(),
                RawAtomExpr::Symbol(_) => todo!(),
                RawAtomExpr::Uncertain => todo!(),
            },
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                RawOpnVariant::Binary(_) => todo!(),
                RawOpnVariant::Prefix(_) => todo!(),
                RawOpnVariant::Suffix(_) => todo!(),
                RawOpnVariant::List(_) => todo!(),
                RawOpnVariant::Field(_) => todo!(),
                RawOpnVariant::CurlBracketed => self.resolve_entity(&self.arena[opds.start]),
                RawOpnVariant::Abstraction => todo!(),
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedToken {
    kind: ResolvedTokenKind,
    range: TextRange,
}

impl TextRanged for ResolvedToken {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl ResolvedToken {
    pub(super) fn kind(&self) -> &ResolvedTokenKind {
        &self.kind
    }

    pub(super) fn to_expr(self, arena: &ExprArena) -> Expr {
        let variant = match self.kind {
            ResolvedTokenKind::Atom(variant) => variant.into(),
            ResolvedTokenKind::BinaryOpr(_) => todo!(),
            ResolvedTokenKind::Prefix(_) => todo!(),
            ResolvedTokenKind::Suffix(_) => todo!(),
        };
        Expr::new(variant, self.range, arena)
    }
}

#[derive(Clone)]
pub(crate) enum ResolvedTokenKind {
    Atom(RawAtomExpr),
    BinaryOpr(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(RawSuffixOpr),
}

impl From<Symbol> for ResolvedTokenKind {
    fn from(symbol: Symbol) -> Self {
        ResolvedTokenKind::Atom(symbol.into())
    }
}
