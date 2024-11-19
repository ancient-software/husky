use super::*;
use crate::helpers::tracker::VdSynExprTracker;
use expect_test::{expect, Expect};
use latex_prelude::{helper::tracker::LxDocumentBodyInput, mode::LxMode};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

fn t(content: &str, expected: &Expect) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;

    let db = &DB::default();
    let file_path = LxFilePath::new(db, PathBuf::from(file!()));
    let tracker = VdSynExprTracker::new(LxDocumentBodyInput { file_path, content }, &[], &[], db);
    expected.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn parse_vd_syn_divisions_works() {
    t(
        r#"Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ "Let $x\\in\\mathbb{R}$." division.stmts
              └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                  └─ "Let $x\\in\\mathbb{R}$" clause.let
                    └─ "x\\in\\mathbb{R}" expr.separated_list
                      ├─ "x" expr.letter
                      └─ "\\mathbb{R}" expr.letter
        "#]],
    );
    t(
        r#"\section{Introduction}Let $x\in\mathbb{R}$."#,
        &expect![[r#"
            └─ "\\section{Introduction}Let $x\\in\\mathbb{R}$." division.section
              ├─ title
              │ └─ "Introduction" stmt.paragraph
              │   └─ "Introduction" sentence.clauses
              │     └─ "Introduction" clause.todo
              └─ "Let $x\\in\\mathbb{R}$." division.stmts
                └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                  └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                    └─ "Let $x\\in\\mathbb{R}$" clause.let
                      └─ "x\\in\\mathbb{R}" expr.separated_list
                        ├─ "x" expr.letter
                        └─ "\\mathbb{R}" expr.letter
        "#]],
    );
    t(
        r#"\section{Introduction}Let $x\in\mathbb{R}$.\subsection{Hello}Let $y\in\mathbb{R}$.\subsection{World}\subsection{This}\subsubsection{Is}\subsubsection{Bad}"#,
        &expect![[r#"
            └─ "\\section{Introduction}Let $x\\in\\mathbb{R}$.\\subsection{Hello}Let $y\\in\\mathbb{R}$.\\subsection{World}\\subsection{This}\\subsubsection{Is}\\subsubsection{Bad}" division.section
              ├─ title
              │ └─ "Introduction" stmt.paragraph
              │   └─ "Introduction" sentence.clauses
              │     └─ "Introduction" clause.todo
              ├─ "Let $x\\in\\mathbb{R}$." division.stmts
              │ └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
              │   └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
              │     └─ "Let $x\\in\\mathbb{R}$" clause.let
              │       └─ "x\\in\\mathbb{R}" expr.separated_list
              │         ├─ "x" expr.letter
              │         └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{Hello}Let $y\\in\\mathbb{R}$." division.subsection
              │ ├─ title
              │ │ └─ "Hello" stmt.paragraph
              │ │   └─ "Hello" sentence.clauses
              │ │     └─ "Hello" clause.todo
              │ └─ "Let $y\\in\\mathbb{R}$." division.stmts
              │   └─ "Let $y\\in\\mathbb{R}$." stmt.paragraph
              │     └─ "Let $y\\in\\mathbb{R}$." sentence.clauses
              │       └─ "Let $y\\in\\mathbb{R}$" clause.let
              │         └─ "y\\in\\mathbb{R}" expr.separated_list
              │           ├─ "y" expr.letter
              │           └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{World}" division.subsection
              │ └─ title
              │   └─ "World" stmt.paragraph
              │     └─ "World" sentence.clauses
              │       └─ "World" clause.todo
              └─ "\\subsection{This}\\subsubsection{Is}\\subsubsection{Bad}" division.subsection
                ├─ title
                │ └─ "This" stmt.paragraph
                │   └─ "This" sentence.clauses
                │     └─ "This" clause.todo
                ├─ "\\subsubsection{Is}" division.subsubsection
                │ └─ title
                │   └─ "Is" stmt.paragraph
                │     └─ "Is" sentence.clauses
                │       └─ "Is" clause.todo
                └─ "\\subsubsection{Bad}" division.subsubsection
                  └─ title
                    └─ "Bad" stmt.paragraph
                      └─ "Bad" sentence.clauses
                        └─ "Bad" clause.todo
        "#]],
    );
}
