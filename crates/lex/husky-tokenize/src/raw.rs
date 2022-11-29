use crate::{word::new_reserved_word, *};
use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::{CharIter, TextCharIter, TextIndent, TextRange};
use husky_token::{SpecialToken, TokenKind};
use std::str::FromStr;

pub(crate) struct RawToken {
    pub(crate) range: TextRange,
    pub(crate) variant: RawTokenVariant,
}

impl RawToken {
    fn new(i: u32, start: u32, end: u32, variant: RawTokenVariant) -> Self {
        RawToken {
            range: husky_text::new_same_line(i, start, end),
            variant,
        }
    }
}

pub(crate) enum RawTokenVariant {
    Certain(TokenKind),
    Literal(RawLiteralData),
    IllFormedLiteral(RawLiteralData),
    SubOrMinus,
}

impl From<TokenKind> for RawTokenVariant {
    fn from(kind: TokenKind) -> Self {
        RawTokenVariant::Certain(kind)
    }
}

impl From<SpecialToken> for RawTokenVariant {
    fn from(value: SpecialToken) -> Self {
        RawTokenVariant::Certain(value.into())
    }
}

impl From<Token> for RawToken {
    fn from(value: Token) -> Self {
        Self {
            range: value.range,
            variant: RawTokenVariant::Certain(value.kind),
        }
    }
}

pub(crate) struct RawTokenIter<'a, 'b> {
    db: &'a dyn WordDb,
    buffer: String,
    char_iter: TextCharIter<'b>,
}

impl<'a, 'b> RawTokenIter<'a, 'b> {
    pub fn new(word_interner: &'a dyn WordDb, input: &'b str) -> Self {
        let mut buffer = String::new();
        buffer.reserve_exact(100);
        Self {
            db: word_interner,
            buffer,
            char_iter: TextCharIter::new(input),
        }
    }
}

impl<'token_line, 'lex: 'token_line> RawTokenIter<'token_line, 'lex> {
    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.char_iter.peek() {
            self.char_iter.next();
        }
    }

    fn next_word(&mut self) -> RawTokenVariant {
        while let Some(c) = self.char_iter.peek() {
            if is_word_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        let len = self.buffer.len();
        self.take_buffer_word()
    }

    fn next_number(&mut self) -> RawTokenVariant {
        while self.peek_char().is_digit(10) {
            self.eat_char()
        }
        match self.peek_char() {
            '.' => {
                self.eat_char();
                while self.peek_char().is_digit(10) {
                    self.eat_char()
                }
                match self.peek_char() {
                    'f' => todo!(),
                    _ => (),
                }
                let len = self.buffer.len();
                RawTokenVariant::Literal(RawLiteralData::Float(self.take_buffer::<f64>().into()))
                    .into()
            }
            'b' => {
                // b32 or b64
                self.ignore_char();
                match self.peek_char() {
                    '3' => {
                        self.ignore_char();
                        if self.peek_char() != '2' {
                            RawTokenVariant::IllFormedLiteral(RawLiteralData::Bits(
                                self.take_buffer::<u64>().into(),
                            ))
                        } else {
                            // b32
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                RawTokenVariant::Literal(RawLiteralData::B32(
                                    self.take_buffer::<u32>().into(),
                                ))
                            }
                        }
                    }
                    '6' => {
                        self.ignore_char();
                        if self.peek_char() != '4' {
                            RawTokenVariant::IllFormedLiteral(RawLiteralData::Bits(
                                self.take_buffer::<u64>().into(),
                            ))
                        } else {
                            // b64
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                RawTokenVariant::Literal(RawLiteralData::B64(
                                    self.take_buffer::<u64>().into(),
                                ))
                            }
                        }
                    }
                    _ => RawTokenVariant::IllFormedLiteral(RawLiteralData::B64(
                        self.take_buffer::<u64>(),
                    )),
                }
            }
            'i' => {
                // i32 or i64
                self.ignore_char();
                match self.peek_char() {
                    '3' => {
                        self.ignore_char();
                        if self.peek_char() != '2' {
                            RawTokenVariant::IllFormedLiteral(RawLiteralData::Integer(
                                self.take_buffer::<i32>().into(),
                            ))
                        } else {
                            // i32
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                RawTokenVariant::Literal(RawLiteralData::I32(
                                    self.take_buffer::<i32>().into(),
                                ))
                            }
                        }
                    }
                    '6' => {
                        self.ignore_char();
                        if self.peek_char() != '4' {
                            RawTokenVariant::IllFormedLiteral(RawLiteralData::Integer(
                                self.take_buffer::<i64>().into(),
                            ))
                        } else {
                            // b64
                            self.ignore_char();
                            if is_word_char(self.peek_char()) {
                                todo!()
                            } else {
                                RawTokenVariant::Literal(RawLiteralData::I64(
                                    self.take_buffer::<i64>().into(),
                                ))
                            }
                        }
                    }
                    _ => RawTokenVariant::IllFormedLiteral(RawLiteralData::I64(
                        self.take_buffer::<i64>(),
                    )),
                }
            }
            default => {
                if default.is_alphabetic() {
                    // letter other than 'b' or 'i' after integer literal is not allowed
                    let mut token_len = self.buffer.len() + 1;
                    while self.peek_char().is_alphabetic() {
                        self.ignore_char();
                        token_len += 1;
                    }
                    RawTokenVariant::IllFormedLiteral(RawLiteralData::B64(
                        self.take_buffer::<u64>().into(),
                    ))
                } else {
                    // integer
                    let len = self.buffer.len();
                    RawTokenVariant::Literal(RawLiteralData::Integer(
                        self.take_buffer::<i32>().into(),
                    ))
                }
            }
        }
    }

    fn take_buffer_word(&mut self) -> RawTokenVariant {
        let word = std::mem::take(&mut self.buffer);
        self.new_word(word)
    }

    fn new_word(&self, word: String) -> RawTokenVariant {
        if let Some(token_kind) = new_reserved_word(&word) {
            // ad hoc
            token_kind.into()
        } else {
            TokenKind::Identifier(self.db.it_ident_owned(word)).into()
        }
    }

    fn take_buffer<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> char {
        if let Some(c) = self.char_iter.peek() {
            c
        } else {
            0.into()
        }
    }

    fn pass_two(&mut self, special: SpecialToken) -> SpecialToken {
        self.char_iter.next();
        special
    }

    fn eat_char(&mut self) {
        let c = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn ignore_char(&mut self) {
        let c = self.char_iter.next().expect("what");
    }

    fn next_special(&mut self, c_start: char) -> Option<RawTokenVariant> {
        Some(
            match c_start {
                '=' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Eq,
                    ))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::Assign(None)),
                },
                ':' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::DeriveAssign),
                    ':' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution)),
                    _ => SpecialToken::Colon,
                },
                '(' => SpecialToken::Bra(Bracket::Par),
                '[' => SpecialToken::Bra(Bracket::Box),
                '{' => SpecialToken::Bra(Bracket::Curl),
                ')' => SpecialToken::Ket(Bracket::Par),
                ']' => SpecialToken::Ket(Bracket::Box),
                '}' => SpecialToken::Ket(Bracket::Curl),
                ',' => SpecialToken::Comma,
                '@' => SpecialToken::At,
                '&' => match self.peek_char() {
                    '&' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::ShortcuitLogic(
                        BinaryShortcuitLogicOpr::And,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::BitAnd,
                    )))),
                    _ => SpecialToken::Ambersand,
                },
                '|' => match self.peek_char() {
                    '|' => self.pass_two(SpecialToken::DoubleVertical),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::BitOr,
                    )))),
                    _ => SpecialToken::Vertical,
                },
                '~' => SpecialToken::BitNot,
                '.' => SpecialToken::FieldAccess,
                ';' => SpecialToken::Semicolon,
                '%' => {
                    SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::RemEuclid))
                }

                '-' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Sub,
                    )))),
                    '-' => self.pass_two(SpecialToken::Decr),
                    '>' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Curry)),
                    _ => return Some(RawTokenVariant::SubOrMinus),
                },
                '<' => match self.peek_char() {
                    '<' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                        BinaryPureClosedOpr::Shl,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Leq,
                    ))),
                    _ => SpecialToken::LAngle,
                },
                '>' => match self.peek_char() {
                    // '>' => self.pass_two(SpecialToken::Shr), // >>
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Geq,
                    ))),
                    _ => SpecialToken::RAngle,
                },
                '*' => match self.peek_char() {
                    '*' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::PureClosed(
                        BinaryPureClosedOpr::Power,
                    ))),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Mul,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Mul)),
                },
                '/' => match self.peek_char() {
                    '/' => return None,
                    '>' => self.pass_two(SpecialToken::XmlKet),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Div,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Div)),
                },
                '+' => match self.peek_char() {
                    '+' => self.pass_two(SpecialToken::Incr),
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Assign(Some(
                        BinaryPureClosedOpr::Add,
                    )))),
                    _ => SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Add)),
                },
                '!' => match self.peek_char() {
                    '=' => self.pass_two(SpecialToken::BinaryOpr(BinaryOpr::Comparison(
                        BinaryComparisonOpr::Neq,
                    ))),
                    '!' => self.pass_two(SpecialToken::DoubleExclamation),
                    _ => SpecialToken::Exclamation,
                },
                '?' => SpecialToken::QuestionMark,
                c => return Some(TokenKind::Unrecognized(c).into()),
            }
            .into(),
        )
    }

    fn next_token_variant(&mut self) -> Option<RawTokenVariant> {
        let c = self.char_iter.next()?;
        if c == ' ' {
            unreachable!()
        } else if c.is_alphabetic() || c == '_' {
            self.buffer.push(c);
            Some(self.next_word())
        } else if c.is_digit(10) {
            self.buffer.push(c);
            Some(self.next_number())
        } else {
            self.next_special(c)
        }
    }
}

impl<'token_line, 'lex: 'token_line> Iterator for RawTokenIter<'token_line, 'lex> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.char_iter.peek()?;
        match c {
            ' ' => {
                self.skip_whitespaces();
                self.next()
            }
            '\n' => self.next(),
            _ => {
                let start = self.char_iter.current_position();
                let variant = self.next_token_variant()?;
                Some(RawToken {
                    range: (start..self.char_iter.current_position()).into(),
                    variant,
                })
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
