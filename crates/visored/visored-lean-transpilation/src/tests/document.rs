use super::*;
use helpers::tracker::VdLeanTranspilationTracker;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxPageInput},
    mode::LxMode,
};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(content: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    let db = &DB::default();
    let file_path = LxFilePath::new(db, PathBuf::from(file!()));
    let tracker =
        VdLeanTranspilationTracker::new(LxDocumentInput { file_path, content }, &[], &[], db);
    expected_display_tree.assert_eq(&tracker.show_display_tree(db));
    expected_fmt.assert_eq(&tracker.show_fmt(db));
}

#[test]
fn basic_document_to_vd_mir_works() {
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `paragraph`
                └─ group: `sentence`
                  └─ variable: `x`
        "#]],
        &expect!["variable x : ℝ"],
    );
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
\section{Introduction}
Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `division`
                └─ group: `paragraph`
                  └─ group: `sentence`
                    └─ variable: `x`
        "#]],
        &expect![[r#"
            namespace Section1
            variable x : ℝ
            end Section1
        "#]],
    );
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
\section{Introduction}
Let $x\in\mathbb{R}$.
\subsection{Hello}
Let $y\in\mathbb{R}$.
\subsection{World}
\subsection{This}
\subsubsection{Is}
\subsubsection{Bad}
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              ├─ group: `division`
              │ └─ group: `paragraph`
              │   └─ group: `sentence`
              │     └─ variable: `x`
              ├─ group: `division`
              │ └─ group: `division`
              │   └─ group: `paragraph`
              │     └─ group: `sentence`
              │       └─ variable: `y`
              ├─ group: `division`
              └─ group: `division`
                ├─ group: `division`
                └─ group: `division`
        "#]],
        &expect![[r#"
            namespace Section1
            variable x : ℝ

            namespace Subsection1
            variable y : ℝ
            end Subsection1

            namespace Subsection2
            end Subsection2

            namespace Subsection3
            namespace Subsubsection1
            end Subsubsection1

            namespace Subsubsection2
            end Subsubsection2
            end Subsection3
            end Section1
        "#]],
    );
}
