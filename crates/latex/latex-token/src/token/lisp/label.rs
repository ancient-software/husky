use super::*;
use eterned::db::EternerDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LxLispXlabel(BaseCoword);

impl LxLispXlabel {
    pub fn new(s: &str, db: &EternerDb) -> Self {
        fn is_valid_ident(s: &str) -> bool {
            s.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == ':')
        }
        debug_assert!(is_valid_ident(s));
        Self(BaseCoword::from_ref(s, db))
    }
}
