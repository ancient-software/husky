use super::*;
use crate::helpers::tracker::VdSemExprTracker;
use expect_test::{expect, Expect};
use latex_prelude::{helper::tracker::LxFormulaInput, mode::LxMode};

pub(crate) fn t(input: &str, expected: &Expect) {
    let db = &DB::default();
    let tracker = VdSemExprTracker::new(LxFormulaInput(input), &[], &[], db);
    expected.assert_eq(&tracker.show_display_tree(db))
}

#[test]
fn basic_vd_sem_expr_works() {
    t(
        "1",
        &expect![[r#"
            "1" expr.literal
        "#]],
    );
    t(
        "+1",
        &expect![[r#"
            "+1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
    t(
        "-1",
        &expect![[r#"
            "-1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
    t(
        "1+1",
        &expect![[r#"
        "1+1" expr.separated_list
        ├─ "1" expr.literal
        └─ "1" expr.literal
    "#]],
    );
    t(
        "1+1=2",
        &expect![[r#"
            "1+1=2" expr.separated_list
            ├─ "1+1" expr.separated_list
            │ ├─ "1" expr.literal
            │ └─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
    t(
        "1<2",
        &expect![[r#"
            "1<2" expr.separated_list
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn frac_vd_sem_expr_works() {
    t(
        "\\frac{1}{2}",
        &expect![[r#"
            "\\frac{1}{2}" fraction
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn sqrt_vd_sem_expr_works() {
    t(
        "\\sqrt{1}",
        &expect![[r#"
        "\\sqrt{1}" sqrt
        └─ "1" expr.literal
    "#]],
    );
}
