use super::*;
use crate::test_helpers::example::LxAstExample;
use expect_test::Expect;

fn t(input: &str, expected: Expect) {
    let db = &DB::default();
    let example = LxAstExample::new(input, LxMode::Math, db);
    let show = example.show(db);
    expected.assert_eq(&show);
}

#[test]
fn parse_basic_math_latex_input_into_asts_then_show_works() {
    t(
        "x",
        expect![[r#"
            x
            └─ x
        "#]],
    );
    t(
        "x+1",
        expect![[r#"
            x+1
            ├─ x
            ├─ +
            └─ 1
        "#]],
    );
    t(
        "x^2",
        expect![[r#"
            x^2
            └─ x^2
              ├─ x
              └─ 2
        "#]],
    );
    t(
        "x_2",
        expect![[r#"
            x_2
            └─ x_2
              ├─ x
              └─ 2
        "#]],
    );
    t(
        "x^{i+2}",
        expect![[r#"
            x^{i+2}
            └─ x^{i+2}
              ├─ x
              └─ {i+2}
                ├─ i
                ├─ +
                └─ 2
        "#]],
    );
}

#[test]
fn parse_math_extended_letter_command_latex_input_into_asts_then_show_works() {
    t(
        "\\alpha",
        expect![[r#"
            \alpha
            └─ \alpha
        "#]],
    );
    t(
        "\\beta",
        expect![[r#"
            \beta
            └─ \beta
        "#]],
    );
    t(
        "\\gamma",
        expect![[r#"
            \gamma
            └─ \gamma
        "#]],
    );
    t(
        "\\pi",
        expect![[r#"
            \pi
            └─ \pi
        "#]],
    );
    t(
        "\\alpha+\\beta",
        expect![[r#"
            \alpha+\beta
            ├─ \alpha
            ├─ +
            └─ \beta
        "#]],
    );
}
