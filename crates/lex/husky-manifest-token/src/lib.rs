#[cfg(test)]
mod tests;

use husky_print_utils::p;
use husky_text_span::TextSpan;
use husky_word::{Word, WordDb};
use std::char;
use std::str;
use std::string;
use std::{borrow::Cow, sync::Arc};

type StringValue = Arc<String>;

#[derive(Eq, PartialEq, Debug)]
pub struct Token<'a> {
    span: TextSpan,
    variant: TokenVariant<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TokenVariant<'a> {
    Whitespace(&'a str),
    Newline,
    Comment(&'a str),

    Equals,
    Period,
    Comma,
    Colon,
    Plus,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Keylike(Word),
    StringLiteral { val: StringValue, multiline: bool },
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    InvalidCharInString(usize, char),
    InvalidEscape(usize, char),
    InvalidHexEscape(usize, char),
    InvalidEscapeValue(usize, u32),
    NewlineInString(usize),
    Unexpected(usize, char),
    UnterminatedString(usize),
    NewlineInTableKey(usize),
    MultilineStringKey(usize),
    Wanted {
        at: usize,
        expected: &'static str,
        found: &'static str,
    },
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    db: &'a dyn WordDb,
    input: &'a str,
    chars: CrlfFold<'a>,
}

#[derive(Clone)]
struct CrlfFold<'a> {
    chars: std::str::CharIndices<'a>,
}

#[derive(Debug)]
enum MaybeString {
    NotEscaped(usize),
    Owned(std::string::String),
}

impl<'a> Tokenizer<'a> {
    pub fn new(db: &'a dyn WordDb, input: &'a str) -> Tokenizer<'a> {
        let mut t = Tokenizer {
            db,
            input,
            chars: CrlfFold {
                chars: input.char_indices(),
            },
        };
        // Eat utf-8 BOM
        t.eatc('\u{feff}');
        t
    }

    pub fn next(&mut self) -> Result<Option<Token<'a>>, Error> {
        let (start, variant) = match self.take_one_char() {
            Some((start, '\n')) => (start, TokenVariant::Newline),
            Some((start, ' ')) => (start, self.whitespace_token(start)),
            Some((start, '\t')) => (start, self.whitespace_token(start)),
            Some((start, '#')) => (start, self.comment_token(start)),
            Some((start, '=')) => (start, TokenVariant::Equals),
            Some((start, '.')) => (start, TokenVariant::Period),
            Some((start, ',')) => (start, TokenVariant::Comma),
            Some((start, ':')) => (start, TokenVariant::Colon),
            Some((start, '+')) => (start, TokenVariant::Plus),
            Some((start, '{')) => (start, TokenVariant::LeftBrace),
            Some((start, '}')) => (start, TokenVariant::RightBrace),
            Some((start, '[')) => (start, TokenVariant::LeftBracket),
            Some((start, ']')) => (start, TokenVariant::RightBracket),
            Some((start, '\'')) => {
                return self.parse_literal_string(start).map(|variant| {
                    Some(Token {
                        span: self.calc_span(start),
                        variant,
                    })
                })
            }
            Some((start, '"')) => {
                return self.parse_basic_string(start).map(|variant| {
                    Some(Token {
                        span: self.calc_span(start),
                        variant,
                    })
                })
            }
            Some((start, ch)) if is_keylike(ch) => (start, self.keylike(start)),
            Some((start, ch)) => return Err(Error::Unexpected(start, ch)),
            None => return Ok(None),
        };

        let span = self.calc_span(start);
        Ok(Some(Token { span, variant }))
    }

    pub fn peek(&mut self) -> Result<Option<Token<'a>>, Error> {
        self.clone().next()
    }

    pub fn eat(&mut self, expected: TokenVariant<'a>) -> Result<bool, Error> {
        self.eat_spanned(expected).map(|s| s.is_some())
    }

    /// Eat a value, returning it's span if it was consumed.
    pub fn eat_spanned(&mut self, expected: TokenVariant<'a>) -> Result<Option<TextSpan>, Error> {
        let span = match self.peek()? {
            Some(token) if expected == token.variant => token.span,
            Some(_) => return Ok(None),
            None => return Ok(None),
        };

        drop(self.next());
        Ok(Some(span))
    }

    pub fn eat_expect(&mut self, expected: TokenVariant<'a>) -> Result<(), Error> {
        // ignore span
        let _ = self.eat_expect_return_its_span(expected)?;
        Ok(())
    }

    /// Expect the given token returning its span.
    pub fn eat_expect_return_its_span(
        &mut self,
        expected: TokenVariant<'a>,
    ) -> Result<TextSpan, Error> {
        let current = self.current();
        match self.next()? {
            Some(token) => {
                if expected == token.variant {
                    Ok(token.span)
                } else {
                    Err(Error::Wanted {
                        at: current,
                        expected: expected.describe(),
                        found: token.variant.describe(),
                    })
                }
            }
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: expected.describe(),
                found: "eof",
            }),
        }
    }

    pub fn parse_table_key(&mut self) -> Result<(TextSpan, Word), Error> {
        let current = self.current();
        match self.next()? {
            Some(Token {
                span,
                variant: TokenVariant::Keylike(k),
            }) => Ok((span, k.into())),
            Some(Token {
                span,
                variant: TokenVariant::StringLiteral { val, multiline },
            }) => {
                let offset = self.substr_offset(span);
                if multiline {
                    return Err(Error::MultilineStringKey(offset));
                }
                match self.input[span.start..span.end].find('\n') {
                    None => Ok((span, self.db.it_word_borrowed(&val))),
                    Some(i) => Err(Error::NewlineInTableKey(offset + i)),
                }
            }
            Some(token) => Err(Error::Wanted {
                at: current,
                expected: "a table key",
                found: token.variant.describe(),
            }),
            None => Err(Error::Wanted {
                at: self.input.len(),
                expected: "a table key",
                found: "eof",
            }),
        }
    }

    pub fn eat_whitespace(&mut self) -> Result<(), Error> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        Ok(())
    }

    pub fn eat_comment(&mut self) -> Result<bool, Error> {
        if !self.eatc('#') {
            return Ok(false);
        }
        drop(self.comment_token(0));
        self.eat_newline_or_eof().map(|()| true)
    }

    pub fn eat_newline_or_eof(&mut self) -> Result<(), Error> {
        let current = self.current();
        match self.next()? {
            None
            | Some(Token {
                variant: TokenVariant::Newline,
                ..
            }) => Ok(()),
            Some(token) => Err(Error::Wanted {
                at: current,
                expected: "newline",
                found: token.variant.describe(),
            }),
        }
    }

    pub fn skip_to_newline(&mut self) {
        loop {
            match self.take_one_char() {
                Some((_, '\n')) | None => break,
                _ => {}
            }
        }
    }

    fn eatc(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.take_one_char();
                true
            }
            _ => false,
        }
    }

    pub fn current(&self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    fn whitespace_token(&mut self, start: usize) -> TokenVariant<'a> {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        TokenVariant::Whitespace(&self.input[start..self.current()])
    }

    fn comment_token(&mut self, start: usize) -> TokenVariant<'a> {
        while let Some((_, ch)) = self.chars.clone().next() {
            if ch != '\t' && !('\u{20}'..='\u{10ffff}').contains(&ch) {
                break;
            }
            self.take_one_char();
        }
        TokenVariant::Comment(&self.input[start..self.current()])
    }

    #[allow(clippy::type_complexity)]
    fn parse_string(
        &mut self,
        delim: char,
        start: usize,
        new_ch: &mut dyn FnMut(
            &mut Tokenizer<'_>,
            &mut MaybeString,
            bool,
            usize,
            char,
        ) -> Result<(), Error>,
    ) -> Result<TokenVariant<'a>, Error> {
        let mut multiline = false;
        if self.eatc(/* second */ delim) {
            if self.eatc(/* third */ delim) {
                multiline = true;
            } else {
                return Ok(TokenVariant::StringLiteral {
                    val: Default::default(),
                    multiline: false,
                });
            }
        }
        let mut val = MaybeString::NotEscaped(self.current());
        let mut n = 0;
        'outer: loop {
            n += 1;
            match self.take_one_char() {
                Some((i, '\n')) => {
                    if multiline {
                        if self.input.as_bytes()[i] == b'\r' {
                            val.to_owned(&self.input[..i]);
                        }
                        if n == 1 {
                            val = MaybeString::NotEscaped(self.current());
                        } else {
                            val.push('\n');
                        }
                        continue;
                    } else {
                        return Err(Error::NewlineInString(i));
                    }
                }
                Some((mut i, ch)) if ch == delim => {
                    if multiline {
                        if !self.eatc(delim) {
                            val.push(delim);
                            continue 'outer;
                        }
                        if !self.eatc(delim) {
                            val.push(delim);
                            val.push(delim);
                            continue 'outer;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                        if self.eatc(delim) {
                            val.push(delim);
                            i += 1;
                        }
                    }
                    return Ok(TokenVariant::StringLiteral {
                        val: val.into_cow(&self.input[..i]),
                        multiline,
                    });
                }
                Some((i, c)) => new_ch(self, &mut val, multiline, i, c)?,
                None => return Err(Error::UnterminatedString(start)),
            }
        }
    }

    fn parse_literal_string(&mut self, start: usize) -> Result<TokenVariant<'a>, Error> {
        self.parse_string('\'', start, &mut |_me, val, _multi, i, ch| {
            if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') {
                val.push(ch);
                Ok(())
            } else {
                Err(Error::InvalidCharInString(i, ch))
            }
        })
    }

    fn parse_basic_string(&mut self, start: usize) -> Result<TokenVariant<'a>, Error> {
        self.parse_string('"', start, &mut |me, val, multi, i, ch| match ch {
            '\\' => {
                val.to_owned(&me.input[..i]);
                match me.chars.next() {
                    Some((_, '"')) => val.push('"'),
                    Some((_, '\\')) => val.push('\\'),
                    Some((_, 'b')) => val.push('\u{8}'),
                    Some((_, 'f')) => val.push('\u{c}'),
                    Some((_, 'n')) => val.push('\n'),
                    Some((_, 'r')) => val.push('\r'),
                    Some((_, 't')) => val.push('\t'),
                    Some((i, c @ 'u')) | Some((i, c @ 'U')) => {
                        let len = if c == 'u' { 4 } else { 8 };
                        val.push(me.parse_hex(start, i, len)?);
                    }
                    Some((i, c @ ' ')) | Some((i, c @ '\t')) | Some((i, c @ '\n')) if multi => {
                        if c != '\n' {
                            while let Some((_, ch)) = me.chars.clone().next() {
                                match ch {
                                    ' ' | '\t' => {
                                        me.chars.next();
                                        continue;
                                    }
                                    '\n' => {
                                        me.chars.next();
                                        break;
                                    }
                                    _ => return Err(Error::InvalidEscape(i, c)),
                                }
                            }
                        }
                        while let Some((_, ch)) = me.chars.clone().next() {
                            match ch {
                                ' ' | '\t' | '\n' => {
                                    me.chars.next();
                                }
                                _ => break,
                            }
                        }
                    }
                    Some((i, c)) => return Err(Error::InvalidEscape(i, c)),
                    None => return Err(Error::UnterminatedString(start)),
                }
                Ok(())
            }
            ch if ch == '\u{09}' || (('\u{20}'..='\u{10ffff}').contains(&ch) && ch != '\u{7f}') => {
                val.push(ch);
                Ok(())
            }
            _ => Err(Error::InvalidCharInString(i, ch)),
        })
    }

    fn parse_hex(&mut self, start: usize, i: usize, len: usize) -> Result<char, Error> {
        let mut buf = String::with_capacity(len);
        for _ in 0..len {
            match self.take_one_char() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_ascii_hexdigit() => buf.push(ch),
                Some((i, ch)) => return Err(Error::InvalidHexEscape(i, ch)),
                None => return Err(Error::UnterminatedString(start)),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(Error::InvalidEscapeValue(i, val)),
        }
    }

    fn keylike(&mut self, start: usize) -> TokenVariant<'a> {
        while let Some((_, ch)) = self.peek_one() {
            if !is_keylike(ch) {
                break;
            }
            self.take_one_char();
        }
        TokenVariant::Keylike(self.db.it_word_borrowed(&self.input[start..self.current()]))
    }

    pub fn substr_offset(&self, span: TextSpan) -> usize {
        let s = &self.input[span.start..span.end];
        assert!(s.len() <= self.input.len());
        let a = self.input.as_ptr() as usize;
        let b = s.as_ptr() as usize;
        assert!(a <= b);
        b - a
    }

    /// Calculate the span from start to next char
    fn calc_span(&mut self, start: usize) -> TextSpan {
        let end = self
            .peek_one()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        TextSpan { start, end }
    }

    /// Peek one char without consuming it.
    fn peek_one(&self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    /// Take one char.
    pub fn take_one_char(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
}

impl<'a> Iterator for CrlfFold<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(i, c)| {
            if c == '\r' {
                let mut attempt = self.chars.clone();
                if let Some((_, '\n')) = attempt.next() {
                    self.chars = attempt;
                    return (i, '\n');
                }
            }
            (i, c)
        })
    }
}

impl MaybeString {
    fn push(&mut self, ch: char) {
        match *self {
            MaybeString::NotEscaped(..) => {}
            MaybeString::Owned(ref mut s) => s.push(ch),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_owned(&mut self, input: &str) {
        match *self {
            MaybeString::NotEscaped(start) => {
                *self = MaybeString::Owned(input[start..].to_owned());
            }
            MaybeString::Owned(..) => {}
        }
    }

    fn into_cow(self, input: &str) -> StringValue {
        match self {
            MaybeString::NotEscaped(start) => Arc::new(input[start..].to_owned()),
            MaybeString::Owned(s) => Arc::new(s),
        }
    }
}

fn is_keylike(ch: char) -> bool {
    ('A'..='Z').contains(&ch)
        || ('a'..='z').contains(&ch)
        || ('0'..='9').contains(&ch)
        || ch == '-'
        || ch == '_'
}

impl<'a> TokenVariant<'a> {
    pub fn describe(&self) -> &'static str {
        match *self {
            TokenVariant::Keylike(_) => "an keylike",
            TokenVariant::Equals => "an equals",
            TokenVariant::Period => "a period",
            TokenVariant::Comment(_) => "a comment",
            TokenVariant::Newline => "a newline",
            TokenVariant::Whitespace(_) => "whitespace",
            TokenVariant::Comma => "a comma",
            TokenVariant::RightBrace => "a right brace",
            TokenVariant::LeftBrace => "a left brace",
            TokenVariant::RightBracket => "a right bracket",
            TokenVariant::LeftBracket => "a left bracket",
            TokenVariant::StringLiteral { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
            TokenVariant::Colon => "a colon",
            TokenVariant::Plus => "a plus",
        }
    }
}
