use text::RangedCustomIdentifier;
use vm::PrimitiveValue;

use super::*;

impl<'a> AtomParser<'a> {
    pub(crate) fn special(&mut self, target: Special) -> Option<()> {
        self.kind(target.into())
    }

    pub(crate) fn usize_literal(&mut self) -> Option<usize> {
        if let Some(Token {
            kind: TokenKind::PrimitiveLiteral(PrimitiveValue::I32(i)),
            ..
        }) = self.stream.next()
        {
            if *i < 0 {
                None
            } else {
                Some(*i as usize)
            }
        } else {
            None
        }
    }

    pub(crate) fn custom_ident(&mut self) -> Option<RangedCustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
            range,
        }) = self.stream.next()
        {
            Some(RangedCustomIdentifier {
                ident: *ident,
                range: *range,
            })
        } else {
            None
        }
    }

    fn kind(&mut self, target: TokenKind) -> Option<()> {
        if let Some(Token { kind, .. }) = self.stream.next() {
            if *kind == target {
                return Some(());
            }
        }
        None
    }
}
