use vec_like::VecPairMap;

use super::*;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn reserved_words(db: &dyn TokenDb) -> VecPairMap<&'static str, RawTokenVariant> {
    macro_rules! gen_reserved_words {
        ($($args: expr),*,) => {
            [
                $(($args.as_str(), $args.into())),*
            ]
        };
    }
    gen_reserved_words![
        Keyword::Main,
        Keyword::Use,
        Keyword::Mod,
        Keyword::Trait,
        Keyword::Impl,
        Keyword::Visual,
        ConfigKeyword::Task,
        FormKeyword::Def,
        FormKeyword::Func,
        FormKeyword::Proc,
        FormKeyword::Fn,
        FormKeyword::Function,
        FormKeyword::Theorem,
        FormKeyword::Lemma,
        FormKeyword::Proposition,
        StmtKeyword::Let,
        StmtKeyword::Var,
        StmtKeyword::Elif,
        StmtKeyword::Else,
        StmtKeyword::If,
        StmtKeyword::Match,
        StmtKeyword::ForExt,
        StmtKeyword::While,
        StmtKeyword::Do,
        StmtKeyword::Break,
        StmtKeyword::Return,
        StmtKeyword::Assert,
        StmtKeyword::Require,
        TypeKeyword::Type,
        TypeKeyword::Struct,
        TypeKeyword::Enum,
        TypeKeyword::Record,
        TypeKeyword::Structure,
        TypeKeyword::Inductive,
        LiasonKeyword::Mut,
        AttrKeyword::Pub,
        AttrKeyword::Private,
        AttrKeyword::Async,
        AttrKeyword::Static,
        WordOpr::And,
        WordOpr::Or,
        WordOpr::As,
        WordOpr::Be,
    ]
    .into_iter()
    .collect()
}
