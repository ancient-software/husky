use super::*;
use husky_opn_syntax::{BinaryOpr, Bracket};

// punctuation in general

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PunctuationToken {
    punc: Punctuation,
    token_idx: TokenIdx,
}

impl<'a, Context> parsec::ParseFrom<Context> for PunctuationToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
            match token {
                Token::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

// specific punctuation

fn parse_specific_punctuation_from<'a, Context, T>(
    ctx: &mut Context,
    target: Punctuation,
    f: impl FnOnce(TokenIdx) -> T,
) -> TokenResult<Option<T>>
where
    Context: TokenParseContext<'a>,
{
    if let Some((token_idx, token)) = ctx.borrow_mut().next_indexed() {
        match token {
            Token::Punctuation(punc) if punc == target => Ok(Some(f(token_idx))),
            Token::Error(error) => Err(error),
            Token::Label(_)
            | Token::Punctuation(_)
            | Token::Ident(_)
            | Token::WordOpr(_)
            | Token::Literal(_)
            | Token::Attr(_)
            | Token::Keyword(_) => Ok(None),
        }
    } else {
        Ok(None)
    }
}

// colon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for ColonToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Colon, ColonToken)
    }
}

#[test]
fn colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ",").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// comma

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommaToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for CommaToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Comma, CommaToken)
    }
}

#[test]
fn comma_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CommaToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ",").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// assign

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssignToken(TokenIdx);

impl AssignToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for AssignToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Eq, AssignToken)
    }
}

#[test]
fn assign_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<AssignToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "=").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftParenthesisToken(TokenIdx);

impl LeftParenthesisToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftParenthesisToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Par), LeftParenthesisToken)
    }
}

#[test]
fn left_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "(").unwrap().is_some());
    assert!(t(&db, ")").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right parenthesis

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightParenthesisToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for RightParenthesisToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Par), RightParenthesisToken)
    }
}

#[test]
fn right_parenthesis_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightParenthesisToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ")").unwrap().is_some());
    assert!(t(&db, "(").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftBoxBracketToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for LeftBoxBracketToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Box), LeftBoxBracketToken)
    }
}

#[test]
fn left_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "[").unwrap().is_some());
    assert!(t(&db, "]").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right box bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightBoxBracketToken(TokenIdx);

impl RightBoxBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for RightBoxBracketToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Box), RightBoxBracketToken)
    }
}

#[test]
fn right_box_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightBoxBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "]").unwrap().is_some());
    assert!(t(&db, "[").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftCurlyBraceToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for LeftCurlyBraceToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Bra(Bracket::Curl), LeftCurlyBraceToken)
    }
}

#[test]
fn left_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "{").unwrap().is_some());
    assert!(t(&db, "}").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightCurlyBraceToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for RightCurlyBraceToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Ket(Bracket::Curl), RightCurlyBraceToken)
    }
}

#[test]
fn right_curly_brace_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightCurlyBraceToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "}").unwrap().is_some());
    assert!(t(&db, "{").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftAngleBracketOrLessThanToken(TokenIdx);

impl LeftAngleBracketOrLessThanToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for LeftAngleBracketOrLessThanToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::LaOrLt, LeftAngleBracketOrLessThanToken)
    }
}

#[test]
fn left_angle_or_less_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<LeftAngleBracketOrLessThanToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "<").unwrap().is_some());
    assert!(t(&db, "::<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// colon colon left angle bracket

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColonColonLeftAngleBracketToken(TokenIdx);

impl ColonColonLeftAngleBracketToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ColonColonLeftAngleBracketToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(
            ctx,
            Punctuation::ColonColonLAngle,
            ColonColonLeftAngleBracketToken,
        )
    }
}

#[test]
fn colon_colon_left_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<ColonColonLeftAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, "::<").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, ">").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// right curly brace

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RightAngleBracketToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for RightAngleBracketToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::RaOrGt, RightAngleBracketToken)
    }
}

#[test]
fn right_angle_bracket_token_works() {
    let db = DB::default();
    fn t(db: &DB, input: &str) -> TokenResult<Option<RightAngleBracketToken>> {
        quick_parse(db, input)
    }

    assert!(t(&db, ">").unwrap().is_some());
    assert!(t(&db, "<").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// vertical

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for VerticalToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::Vertical, VerticalToken)
    }
}

#[test]
fn vertical_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<VerticalToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "|").unwrap().is_some());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// at

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for AtToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::At, AtToken)
    }
}

#[test]
fn at_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<AtToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "@").unwrap().is_some());
    assert!(t(&db, "|").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

// dotdot

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DotDotToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for DotDotToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        parse_specific_punctuation_from(ctx, Punctuation::DotDot, DotDotToken)
    }
}

#[test]
fn dotdot_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<DotDotToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "..").unwrap().is_some());
    assert!(t(&db, "@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `:` at the end of line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EolColonToken(TokenIdx);

impl EolColonToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for EolColonToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Colon) => match token_stream.peek() {
                    Some(_) => Ok(None),
                    None => Ok(Some(EolColonToken(token_idx))),
                },
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn eol_colon_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<EolColonToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, ":").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `::`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeResolutionToken(TokenIdx);

impl ScopeResolutionToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for ScopeResolutionToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::ColonColon) => {
                    Ok(Some(ScopeResolutionToken(token_idx)))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn scope_resolution_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<ScopeResolutionToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "::").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_some());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `*`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarToken(TokenIdx);

impl StarToken {
    pub fn token_idx(&self) -> TokenIdx {
        self.0
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for StarToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Star) => Ok(Some(StarToken(token_idx))),
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn star_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<StarToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "*").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}

/// `->`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurryToken(TokenIdx);

impl<'a, Context> parsec::ParseFrom<Context> for CurryToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from_without_guaranteed_rollback(ctx: &mut Context) -> TokenResult<Option<Self>> {
        let token_stream = ctx.token_stream_mut();
        if let Some((token_idx, token)) = token_stream.next_indexed() {
            match token {
                Token::Punctuation(Punctuation::Binary(BinaryOpr::Curry)) => {
                    Ok(Some(CurryToken(token_idx)))
                }
                Token::Error(error) => Err(error),
                Token::Label(_)
                | Token::Punctuation(_)
                | Token::Ident(_)
                | Token::WordOpr(_)
                | Token::Literal(_)
                | Token::Attr(_)
                | Token::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn curry_token_works() {
    fn t(db: &DB, input: &str) -> TokenResult<Option<CurryToken>> {
        quick_parse(db, input)
    }

    let db = DB::default();
    assert!(t(&db, "->").unwrap().is_some());
    assert!(t(&db, "::@").unwrap().is_none());
    assert!(t(&db, ":@").unwrap().is_none());
    assert!(t(&db, ".").unwrap().is_none());
    assert!(t(&db, "||").unwrap().is_none());
    assert!(t(&db, "a").unwrap().is_none());
    assert!(t(&db, "'").is_err());
}
