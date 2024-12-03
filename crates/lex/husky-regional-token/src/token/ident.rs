use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentRegionalToken {
    pub(crate) ident: Ident,
    pub(crate) regional_token_idx: RegionalTokenIdx,
}

impl IdentRegionalToken {
    pub fn new(ident: Ident, regional_token_idx: RegionalTokenIdx) -> Self {
        Self {
            ident,
            regional_token_idx,
        }
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ident_ref(&self) -> &Ident {
        &self.ident
    }

    pub fn regional_token_idx(&self) -> RegionalTokenIdx {
        self.regional_token_idx
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for IdentRegionalToken
where
    Context: RegionalTokenStreamParser<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((regional_token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => Ok(Some(IdentRegionalToken {
                    ident,
                    regional_token_idx,
                })),
                TokenData::Error(error) => Err(error)?,
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn ident_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnderscoreRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for UnderscoreRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data() {
                    "_" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn underscore_token_works() {
    // todo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum AttrIdentRegionalToken {
    Derive(DeriveRegionalToken),
}

// "derive"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeriveRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for DeriveRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data() {
                    "derive" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn derive_regional_token_works() {
    // todo
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrRegionalToken {
    Phan(PhanRegionalToken),
    Poly(PolyRegionalToken),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PhanRegionalToken {
    token_idx: RegionalTokenIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PolyRegionalToken {
    token_idx: RegionalTokenIdx,
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for AttrRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data() {
                    "phan" => Ok(Some(AttrRegionalToken::Phan(PhanRegionalToken {
                        token_idx,
                    }))),
                    "poly" => Ok(Some(AttrRegionalToken::Poly(PolyRegionalToken {
                        token_idx,
                    }))),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PhanRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data() {
                    "phantom" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a, Context> parsec::TryParseOptionFromStream<Context> for PolyRegionalToken
where
    Context: RegionalTokenStreamParser<'a> + ::salsa::db::HasDb<'a>,
{
    type Error = TokenDataError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut Context,
    ) -> TokenDataResult<Option<Self>> {
        if let Some((token_idx, token)) = ctx.token_stream_mut().next_indexed() {
            match token {
                TokenData::Ident(ident) => match ident.data() {
                    "runtime" => Ok(Some(Self { token_idx })),
                    _ => Ok(None),
                },
                TokenData::Error(error) => Err(error),
                TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_)
                | TokenData::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
