mod pretoken;
mod reserved;
mod tokenizer;
mod word;

pub(crate) use self::reserved::*;

use self::pretoken::*;
use self::tokenizer::*;
use self::word::*;
use crate::*;
use husky_text_protocol::char_iter::TextCharIter;

// must be used inside tracked context
pub(crate) fn tokenize<'a>(db: &::salsa::Db, input: &str) -> RangedTokenSheet {
    let pretoken_iter = PretokenStream::new(db, TextCharIter::new(input));
    let mut tokenizer = Tokenizer::new(db);
    tokenizer.push_tokens(pretoken_iter);
    tokenizer.finish()
}
