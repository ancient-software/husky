use super::*;
use ast::helpers::tracker::LxAstTracker;
use expect_test::{expect, Expect};
use interned::db::InternerDb;
use latex_prelude::{helper::tracker::LxDocumentInput, mode::LxMode};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(content: &str, expected: Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &InternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let tracker = LxAstTracker::new(
        LxDocumentInput {
            specs_dir: dev_paths.specs_dir().to_path_buf(),
            file_path,
            content,
        },
        db,
    );
    let show = tracker.show(db);
    expected.assert_eq(&show);
}

#[test]
fn parse_use_packages_into_lx_root_asts_works() {
    t(
        "\\documentclass{article}",
        expect![[r#"
            └─ "\\documentclass{article}" complete command
              └─ article
        "#]],
    );
    t(
        "\\usepackage{amsmath}",
        expect![[r#"
            └─ "\\usepackage{amsmath}" complete command
              └─ amsmath
        "#]],
    );
    t(
        "\\usepackage[utf8]{inputenc}",
        expect![[r#"
            └─ "\\usepackage[utf8]{inputenc}" complete command
              └─ inputenc
        "#]],
    );
}

#[test]
fn parse_document_environment_works() {
    t(
        r#"\begin{document}\end{document}"#,
        expect![[r#"
            └─ "\\begin{document}\\end{document}" environment
        "#]],
    );
    t(
        r#"\begin{document}Hello\end{document}"#,
        expect![[r#"
            └─ "\\begin{document}Hello\\end{document}" environment
              └─ "Hello" word
        "#]],
    );
    t(
        r#"\begin{document}Let $x\in\mathbb{R}$.\end{document}"#,
        expect![[r#"
            └─ "\\begin{document}Let $x\\in\\mathbb{R}$.\\end{document}" environment
              ├─ "Let" word
              ├─ "$x\\in\\mathbb{R}$" math
              │ ├─ "x" plain letter
              │ ├─ "\\in" complete command
              │ └─ "\\mathbb{R}" styled letter
              └─ "." punctuation
        "#]],
    );
}
