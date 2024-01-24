use super::*;

use husky_text_protocol::{char_iter::TextCharIter, range::TextRange};

use husky_coword::{is_char_valid_ident_first_char, Label};
use std::str::FromStr;

pub(crate) struct RangedPretoken {
    pub(crate) range: TextRange,
    pub(crate) token: Pretoken,
}

impl RangedPretoken {
    fn new(i: u32, start: u32, end: u32, token: Pretoken) -> Self {
        RangedPretoken {
            range: husky_text_protocol::range::new_same_line(i, start, end),
            token,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Pretoken {
    Certain(TokenData),
    Literal(LiteralData),
    NewLine,
    Ambiguous(AmbiguousPretoken),
    Comment,
    Err(TokenDataError),
}

impl From<AmbiguousPretoken> for Pretoken {
    fn from(v: AmbiguousPretoken) -> Self {
        Self::Ambiguous(v)
    }
}

impl From<IntegerLikeLiteralData> for Pretoken {
    fn from(val: IntegerLikeLiteralData) -> Self {
        Pretoken::Certain(TokenData::Literal(LiteralData::Integer(val)))
    }
}

impl From<FloatLiteralData> for Pretoken {
    fn from(val: FloatLiteralData) -> Self {
        Pretoken::Certain(TokenData::Literal(LiteralData::Float(val)))
    }
}

impl From<EndKeyword> for Pretoken {
    fn from(kw: EndKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AmbiguousPretoken {
    SubOrMinus,
    For,
}

impl AmbiguousPretoken {
    pub fn code(self) -> &'static str {
        match self {
            AmbiguousPretoken::SubOrMinus => todo!(),
            AmbiguousPretoken::For => "for",
        }
    }
}

impl From<TokenData> for Pretoken {
    fn from(kind: TokenData) -> Self {
        Pretoken::Certain(kind)
    }
}

impl From<Punctuation> for Pretoken {
    fn from(value: Punctuation) -> Self {
        Pretoken::Certain(value.into())
    }
}

impl From<Keyword> for Pretoken {
    fn from(kw: Keyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<StmtKeyword> for Pretoken {
    fn from(kw: StmtKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<TypeEntityKeyword> for Pretoken {
    fn from(kw: TypeEntityKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<ModifierKeyword> for Pretoken {
    fn from(kw: ModifierKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<WordOpr> for Pretoken {
    fn from(kw: WordOpr) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<FugitiveKeyword> for Pretoken {
    fn from(val: FugitiveKeyword) -> Self {
        Pretoken::Certain(val.into())
    }
}

impl From<PronounKeyword> for Pretoken {
    fn from(val: PronounKeyword) -> Self {
        Pretoken::Certain(val.into())
    }
}

impl From<ConnectionKeyword> for Pretoken {
    fn from(val: ConnectionKeyword) -> Self {
        Pretoken::Certain(val.into())
    }
}

impl From<BoolLiteralData> for Pretoken {
    fn from(value: BoolLiteralData) -> Self {
        Pretoken::Certain(value.into())
    }
}

pub(crate) struct PretokenStream<'a, 'b> {
    db: &'a ::salsa::Db,
    buffer: String,
    char_iter: TextCharIter<'b>,
}

impl<'a, 'b> PretokenStream<'a, 'b> {
    pub fn new(db: &'a ::salsa::Db, char_iter: TextCharIter<'b>) -> Self {
        let mut buffer = String::new();
        buffer.reserve_exact(100);
        Self {
            db,
            buffer,
            char_iter,
        }
    }
}

impl<'a, 'b: 'a> PretokenStream<'a, 'b> {
    fn next_token_variant(&mut self) -> Option<Pretoken> {
        let c = self.char_iter.next()?;
        assert_ne!(c, ' ');
        match c {
            '\n' => Some(Pretoken::NewLine),
            '\'' => Some(self.next_char_or_lifetime_or_label()),
            '"' => {
                match self.next_string_literal(StringLiteralKind::SingleLine) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        // skip this line
                        while let Some(c) = self.char_iter.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                        Some(Pretoken::Err(e))
                    }
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                self.buffer.push(c);
                Some(self.next_coword())
            }
            c if c.is_digit(10) => {
                self.buffer.push(c);
                Some(self.next_number())
            }
            c if c == '/' && self.char_iter.peek() == Some('/') => {
                while let Some(c) = self.char_iter.peek() {
                    if c == '\n' {
                        break;
                    } else {
                        self.char_iter.next();
                    }
                }
                Some(Pretoken::Comment)
            }
            c => self.next_punctuation(c),
        }
    }

    /// assume a previous single quote has been taken
    fn next_char_or_lifetime_or_label(&mut self) -> Pretoken {
        let Some((fst, snd)) = self.char_iter.peek_two() else {
            return Pretoken::Err(TokenDataError::NothingAfterSingleQuote);
        };
        match fst {
            '\\' => todo!(),
            fst if is_char_valid_ident_first_char(fst) => match snd {
                Some('\'') => {
                    self.char_iter.next();
                    self.char_iter.next();
                    Pretoken::Literal(LiteralData::Char(CharLiteralData::Basic(fst)))
                }
                _ => self.next_auxiliary_identifier(),
            },
            _ => {
                self.char_iter.next();
                Pretoken::Err(TokenDataError::InvalidLabel)
            }
        }
    }

    fn next_auxiliary_identifier(&mut self) -> Pretoken {
        while let Some(c) = self.char_iter.peek() {
            if is_coword_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        assert!(self.buffer.len() > 0);
        let word = &self.buffer;
        let pretoken = match Label::from_ref(self.db, word) {
            Some(ident) => TokenData::Label(ident).into(),
            None => Pretoken::Err(TokenDataError::InvalidIdent),
        };
        self.buffer.clear();
        pretoken
    }

    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.char_iter.peek() {
            self.char_iter.next();
        }
    }

    fn next_coword(&mut self) -> Pretoken {
        while let Some(c) = self.char_iter.peek() {
            if is_coword_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        assert!(self.buffer.len() > 0);
        let word = &self.buffer;
        let pretoken = if let Some(pretoken) = new_reserved_coword(self.db, word) {
            pretoken
        } else {
            match Ident::from_ref(self.db, word) {
                Some(identifier) => TokenData::Ident(identifier).into(),
                None => Pretoken::Err(TokenDataError::InvalidIdent),
            }
        };
        self.buffer.clear();
        pretoken
    }

    fn next_number(&mut self) -> Pretoken {
        let radix = 10;
        self.eat_chars_with(|c| char::is_digit(c, radix));
        if self.try_eat_char(|c| c == '.').is_some() {
            // parse float type
            self.eat_chars_with(|c| c.is_digit(radix));
            let float_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            let token: Pretoken = match float_suffix {
                "" => FloatLiteralData::Unspecified(UnspecifiedFloatLiteral::new(
                    self.db,
                    std::mem::take(&mut self.buffer),
                ))
                .into(),
                "f8" => todo!(),
                "f16" => todo!(),
                "f32" => todo!(),
                "f64" => todo!(),
                "f128" => todo!(),
                "f256" => todo!(),
                _ => Pretoken::Err(TokenDataError::InvalidFloatSuffix),
            };
            self.buffer.clear();
            token
        } else {
            let integer_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            let token: Pretoken = match integer_suffix {
                "" => match self.buffer.parse::<i128>() {
                    Ok(i) => IntegerLikeLiteralData::UnspecifiedRegular(i).into(),
                    Err(_) => todo!(),
                },
                "i8" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::I8(i).into()
                }
                "i16" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::I16(i).into()
                }
                "i32" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::I32(i).into()
                }
                "i64" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::I64(i).into()
                }
                "i128" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::I128(i).into()
                }
                "r8" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::R8(i).into()
                }
                "r16" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::R16(i).into()
                }
                "r32" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::R32(i).into()
                }
                "r64" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::R64(i).into()
                }
                "r128" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::R128(i).into()
                }
                "u8" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::U8(i).into()
                }
                "u16" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::U16(i).into()
                }
                "u32" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::U32(i).into()
                }
                "u64" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::U64(i).into()
                }
                "u128" => {
                    let Ok(i) = self.buffer.parse() else {
                        return Pretoken::Err(TokenDataError::ParseIntError);
                    };
                    IntegerLikeLiteralData::U128(i).into()
                }
                _invalid_integer_suffix => {
                    return Pretoken::Err(TokenDataError::InvalidIntegerSuffix)
                }
            };
            self.buffer.clear();
            token
        }
    }

    fn take_buffer_parsed<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek()
    }

    fn turn_peek_into_next(&mut self, special: Punctuation) -> Punctuation {
        self.char_iter.next();
        special
    }

    fn eat_char(&mut self) {
        let c = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn try_eat_char(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eat_chars_with(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
            } else {
                break;
            }
        }
    }

    fn get_str_slice_with(&mut self, predicate: impl Fn(char) -> bool) -> &'b str {
        self.char_iter.get_str_slice_with(predicate)
    }

    fn ignore_char(&mut self) {
        let _c = self.char_iter.next().expect("what");
    }

    fn next_punctuation(&mut self, c_start: char) -> Option<Pretoken> {
        Some(
            match c_start {
                '=' => match self.peek_char() {
                    Some('=') => self.turn_peek_into_next(Punctuation::EQ_EQ),
                    Some('>') => self.turn_peek_into_next(Punctuation::HEAVY_ARROW),
                    _ => Punctuation::EQ,
                },
                ':' => match self.peek_char() {
                    Some('=') => self.turn_peek_into_next(Punctuation::COLON_EQ),
                    Some(':') => {
                        self.char_iter.next();
                        match self.peek_char() {
                            Some('<') => {
                                self.char_iter.next();
                                Punctuation::COLON_COLON_LA
                            }
                            _ => Punctuation::COLON_COLON,
                        }
                    }
                    _ => Punctuation::COLON,
                },
                '(' => Punctuation::LPAR,
                '[' => Punctuation::LBOX,
                '{' => Punctuation::LCURL,
                ')' => Punctuation::RPAR,
                ']' => Punctuation::RBOX,
                '}' => Punctuation::RCURL,
                ',' => Punctuation::COMMA,
                '@' => match self.peek_char() {
                    Some('=') => self.turn_peek_into_next(Punctuation::AT_EQ),
                    _ => Punctuation::AT,
                },
                '&' => match self.peek_char() {
                    Some('&') => self.turn_peek_into_next(Punctuation::LOGIC_AND),
                    Some('=') => self.turn_peek_into_next(Punctuation::BIT_AND_ASSIGN),
                    _ => Punctuation::AMBERSAND,
                },
                '|' => match self.peek_char() {
                    Some('|') => self.turn_peek_into_next(Punctuation::DOUBLE_VERTICAL),
                    Some('=') => self.turn_peek_into_next(Punctuation::BIT_OR_ASSIGN),
                    _ => Punctuation::VERTICAL,
                },
                '~' => Punctuation::TILDE,
                '.' => match self.peek_char() {
                    Some('.') => {
                        self.char_iter.next();
                        match self.peek_char() {
                            Some('.') => self.turn_peek_into_next(Punctuation::DOT_DOT_DOT),
                            _ => Punctuation::DOT_DOT,
                        }
                    }
                    _ => Punctuation::DOT,
                },
                ';' => Punctuation::SEMICOLON,
                '%' => Punctuation::REM_EUCLID,

                '-' => match self.peek_char() {
                    Some('=') => self.turn_peek_into_next(Punctuation::SUB_ASSIGN),
                    Some('-') => self.turn_peek_into_next(Punctuation::DECR),
                    Some('>') => self.turn_peek_into_next(Punctuation::LIGHT_ARROW),
                    _ => return Some(Pretoken::Ambiguous(AmbiguousPretoken::SubOrMinus)),
                },
                '<' => match self.peek_char() {
                    Some('<') => self.turn_peek_into_next(Punctuation::SHL),
                    Some('=') => self.turn_peek_into_next(Punctuation::LEQ),
                    _ => Punctuation::LA_OR_LT,
                },
                '>' => match self.peek_char() {
                    Some('>') => self.turn_peek_into_next(Punctuation::SHR), // >>
                    Some('=') => self.turn_peek_into_next(Punctuation::GEQ),
                    _ => Punctuation::RA_OR_GT,
                },
                '$' => Punctuation::SHEBA,
                '*' => match self.peek_char() {
                    Some('*') => self.turn_peek_into_next(Punctuation::STAR_STAR),
                    Some('=') => self.turn_peek_into_next(Punctuation::MUL_ASSIGN),
                    _ => Punctuation::STAR,
                },
                '/' => match self.peek_char() {
                    Some('/') => unreachable!(),
                    Some('>') => self.turn_peek_into_next(Punctuation::EMPTY_HTML_KET),
                    Some('=') => self.turn_peek_into_next(Punctuation::DIV_ASSIGN),
                    _ => Punctuation::DIV,
                },
                '+' => match self.peek_char() {
                    Some('+') => self.turn_peek_into_next(Punctuation::INCR),
                    Some('=') => self.turn_peek_into_next(Punctuation::ADD_ASSIGN),
                    _ => Punctuation::ADD,
                },
                '!' => match self.peek_char() {
                    Some('=') => self.turn_peek_into_next(Punctuation::NEQ),
                    Some('!') => self.turn_peek_into_next(Punctuation::DOUBLE_EXCLAMATION),
                    _ => Punctuation::EXCLAMATION,
                },
                '?' => Punctuation::QUESTION,
                '#' => Punctuation::POUND,
                '∀' => Punctuation::FOR_ALL,
                '∃' => Punctuation::EXISTS,
                c => return Some(Pretoken::Err(TokenDataError::UnrecognizedChar(c))),
            }
            .into(),
        )
    }

    fn next_string_literal(
        &mut self,
        string_literal_kind: StringLiteralKind,
    ) -> TokenDataResult<Pretoken> {
        let mut s = String::new();
        while let Some(c) = self.char_iter.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(c) = self.char_iter.next() {
                        match c {
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            c => return Err(TokenDataError::UnexpectedCharAfterBackslash(c)),
                        }
                    } else {
                        return Err(TokenDataError::IncompleteStringLiteralBeforeEof);
                    }
                }
                '\n' => match string_literal_kind {
                    StringLiteralKind::SingleLine => {
                        return Err(TokenDataError::IncompleteStringLiteralBeforeEol);
                    }
                    StringLiteralKind::MultipleLine => s.push('\n'),
                },
                c => s.push(c),
            }
        }
        Ok(Pretoken::Literal(LiteralData::String(
            StringLiteralData::new(self.db, s),
        )))
    }
}

enum StringLiteralKind {
    SingleLine,
    MultipleLine,
}

impl<'token_line, 'lex: 'token_line> Iterator for PretokenStream<'token_line, 'lex> {
    type Item = RangedPretoken;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.char_iter.peek()?;
        match c {
            ' ' => {
                self.skip_whitespaces();
                self.next()
            }
            _ => {
                let start = self.char_iter.current_position();
                let variant = self.next_token_variant()?;
                Some(RangedPretoken {
                    range: (start..self.char_iter.current_position()).into(),
                    token: variant,
                })
            }
        }
    }
}

fn is_coword_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
