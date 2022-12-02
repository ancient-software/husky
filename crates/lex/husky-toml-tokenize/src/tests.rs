use super::{TokenIter, TomlTokenError, TomlTokenVariant};
use crate::*;
use crate::{TomlSpecialToken, TomlTokenizeJar};
use expect_test::expect_file;
use husky_package_path::{PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_source_path::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigMimic, SourcePathJar,
};
use husky_toolchain::ToolchainJar;
use husky_vfs::VfsJar;
use husky_word::{WordDb, WordJar};
use salsa::{Database, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    TomlTokenizeJar,
    VfsJar,
    SourcePathJar
)]
#[derive(Default)]
pub struct MimicDB {
    storage: salsa::Storage<Self>,
    source_path_config: SourcePathConfigMimic,
}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

fn err(input: &str, err: TomlTokenError) {
    let db = MimicDB::default();
    let mut t = TokenIter::new(&db, input);
    let token = t.next().unwrap().variant().clone().unwrap_err();
    assert_eq!(token, err);
    assert!(t.next().is_none());
}

#[test]
fn literal_strings() {
    fn t(db: &dyn WordDb, input: &str, val: &str, multiline: bool) {
        let mut t = TokenIter::new(db, input);
        let token = t.next().unwrap().variant().clone().unwrap();
        assert_eq!(
            token,
            TomlTokenVariant::StringLiteral {
                val: Arc::new(val.to_owned()),
                multiline,
            }
        );
        assert!(t.next().is_none());
    }

    let db = MimicDB::default();
    t(&db, "''", "", false);
    t(&db, "''''''", "", true);
    t(&db, "'''\n'''", "", true);
    t(&db, "'a'", "a", false);
    t(&db, "'\"a'", "\"a", false);
    t(&db, "''''a'''", "'a", true);
    t(&db, "'''\n'a\n'''", "'a\n", true);
    t(&db, "'''a\n'a\r\n'''", "a\n'a\n", true);
}

#[test]
fn basic_strings() {
    fn t(db: &dyn WordDb, input: &str, val: &str, multiline: bool) {
        let mut t = TokenIter::new(db, input);
        let token = t.next().unwrap().variant().clone().unwrap();
        assert_eq!(
            token,
            TomlTokenVariant::StringLiteral {
                val: Arc::new(val.to_owned()),
                multiline,
            }
        );
        assert!(t.next().is_none());
    }

    let db = MimicDB::default();
    t(&db, r#""""#, "", false);
    t(&db, r#""""""""#, "", true);
    t(&db, r#""a""#, "a", false);
    t(&db, r#""""a""""#, "a", true);
    t(&db, r#""\t""#, "\t", false);
    t(&db, r#""\u0000""#, "\0", false);
    t(&db, r#""\U00000000""#, "\0", false);
    t(&db, r#""\U000A0000""#, "\u{A0000}", false);
    t(&db, r#""\\t""#, "\\t", false);
    t(&db, "\"\t\"", "\t", false);
    t(&db, "\"\"\"\n\t\"\"\"", "\t", true);
    t(&db, "\"\"\"\\\n\"\"\"", "", true);
    t(
        &db,
        "\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"",
        "",
        true,
    );
    t(&db, r#""\r""#, "\r", false);
    t(&db, r#""\n""#, "\n", false);
    t(&db, r#""\b""#, "\u{8}", false);
    t(&db, r#""a\fa""#, "a\u{c}a", false);
    t(&db, r#""\"a""#, "\"a", false);
    t(&db, "\"\"\"\na\"\"\"", "a", true);
    t(&db, "\"\"\"\n\"\"\"", "", true);
    t(&db, r#""""a\"""b""""#, "a\"\"\"b", true);
    err(r#""\a"#, TomlTokenError::InvalidEscape(2, 'a'));
    err("\"\\\n", TomlTokenError::InvalidEscape(2, '\n'));
    err("\"\\\r\n", TomlTokenError::InvalidEscape(2, '\n'));
    err("\"\\", TomlTokenError::UnterminatedString);
    err("\"\u{0}", TomlTokenError::InvalidCharInString(1, '\u{0}'));
    err(r#""\U00""#, TomlTokenError::InvalidHexEscape(5, '"'));
    err(r#""\U00"#, TomlTokenError::UnterminatedString);
    err(r#""\uD800"#, TomlTokenError::InvalidEscapeValue(2, 0xd800));
    err(
        r#""\UFFFFFFFF"#,
        TomlTokenError::InvalidEscapeValue(2, 0xffff_ffff),
    );
}

#[test]
fn keylike() {
    fn t(db: &dyn WordDb, input: &str) {
        let mut t = TokenIter::new(db, input);
        let token = t.next().unwrap().variant().clone().unwrap();
        assert_eq!(token, TomlTokenVariant::Keylike(db.it_word_borrowed(input)));
        assert!(t.next().is_none());
    }

    let db = MimicDB::default();
    t(&db, "foo");
    t(&db, "0bar");
    t(&db, "bar0");
    t(&db, "1234");
    t(&db, "a-b");
    t(&db, "a_B");
    t(&db, "-_-");
    t(&db, "___");
}

#[test]
fn all() {
    fn t(db: &dyn WordDb, input: &str, expected: &[((usize, usize), TomlTokenVariant, &str)]) {
        let mut tokens = TokenIter::new(db, input);
        let mut actual: Vec<((usize, usize), TomlTokenVariant, &str)> = Vec::new();
        while let Some(token) = tokens.next() {
            actual.push((
                token.span().into(),
                token.variant().clone().unwrap(),
                &input[token.span().start..token.span().end],
            ));
        }
        for (a, b) in actual.iter().zip(expected) {
            assert_eq!(a, b);
        }
        assert_eq!(actual.len(), expected.len());
    }

    let db = MimicDB::default();
    t(
        &db,
        " a ",
        &[
            ((0, 1), TomlTokenVariant::Whitespace, " "),
            (
                (1, 2),
                TomlTokenVariant::Keylike(db.it_word_borrowed("a")),
                "a",
            ),
            ((2, 3), TomlTokenVariant::Whitespace, " "),
        ],
    );

    t(
        &db,
        " a\t [[]] \t [] {} , . =\n# foo \r\n#foo \n ",
        &[
            ((0, 1), TomlTokenVariant::Whitespace, " "),
            (
                (1, 2),
                TomlTokenVariant::Keylike(db.it_word_borrowed("a")),
                "a",
            ),
            ((2, 4), TomlTokenVariant::Whitespace, "\t "),
            ((4, 5), TomlSpecialToken::LeftBox.into(), "["),
            ((5, 6), TomlSpecialToken::LeftBox.into(), "["),
            ((6, 7), TomlSpecialToken::RightBox.into(), "]"),
            ((7, 8), TomlSpecialToken::RightBox.into(), "]"),
            ((8, 11), TomlTokenVariant::Whitespace, " \t "),
            ((11, 12), TomlSpecialToken::LeftBox.into(), "["),
            ((12, 13), TomlSpecialToken::RightBox.into(), "]"),
            ((13, 14), TomlTokenVariant::Whitespace, " "),
            ((14, 15), TomlSpecialToken::LeftCurly.into(), "{"),
            ((15, 16), TomlSpecialToken::RightCurly.into(), "}"),
            ((16, 17), TomlTokenVariant::Whitespace, " "),
            ((17, 18), TomlSpecialToken::Comma.into(), ","),
            ((18, 19), TomlTokenVariant::Whitespace, " "),
            ((19, 20), TomlSpecialToken::Period.into(), "."),
            ((20, 21), TomlTokenVariant::Whitespace, " "),
            ((21, 22), TomlSpecialToken::Equals.into(), "="),
            ((23, 29), TomlTokenVariant::Comment, "# foo "),
            ((31, 36), TomlTokenVariant::Comment, "#foo "),
            ((37, 38), TomlTokenVariant::Whitespace, " "),
        ],
    );
}

#[test]
fn bare_cr_bad() {
    err("\r", TomlTokenError::UnexpectedChar('\r'));
    err("'\n", TomlTokenError::NewlineInString(1));
    err("'\u{0}", TomlTokenError::InvalidCharInString(1, '\u{0}'));
    err("'", TomlTokenError::UnterminatedString);
    err("\u{0}", TomlTokenError::UnexpectedChar('\u{0}'));
}

#[test]
fn bad_comment() {
    let db = MimicDB::default();
    let mut t = TokenIter::new(&db, "#\u{0}");
    t.next().unwrap();
    assert_eq!(
        t.next().unwrap().variant(),
        &Err(TomlTokenError::UnexpectedChar('\u{0}'))
    );
    assert!(t.next().is_none());
}

#[test]
fn builtin_library_toml_token_sheets() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
    expect_file!["../tests/package_core_toml_token_sheets.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_token_text(package_path_menu.core())
            .as_ref()
            .unwrap()
    ));
    expect_file!["../tests/package_std_toml_token_sheets.txt"].assert_eq(&format!(
        "{:#?}",
        db.package_manifest_token_text(package_path_menu.std())
            .as_ref()
            .unwrap()
    ));
}
