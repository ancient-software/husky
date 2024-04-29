mod context;
mod ident;
mod keyword;
mod label;
mod path_name;
mod punctuation;
mod symbol_modifier;
pub mod tokra;

pub use self::context::*;
pub use self::ident::*;
pub use self::keyword::*;
pub use self::label::*;
pub use self::path_name::*;
pub use self::punctuation::*;
pub use self::symbol_modifier::*;

use crate::*;
use parsec::*;

#[cfg(test)]
fn quick_parse<T, Error>(db: &::salsa::Db, input: &str) -> Result<Option<T>, Error>
where
    T: for<'a> TryParseOptionFromStream<TokenStream<'a>, Error = Error>,
{
    use husky_vfs::script::Script;

    let token_sheet = db.snippet_token_sheet_data(Script::new_dev_snippet(input, db));
    let mut stream = token_sheet
        .token_verse_token_stream(token_sheet.main_token_verse_iter().next().unwrap().0, None);
    stream.try_parse_option()
}
