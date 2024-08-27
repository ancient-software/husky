//! Test that if field X of an input changes but not field Y,
//! functions that depend on X re-execute, but those depending only on Y do not
//! compiles and executes successfully.
#![allow(dead_code)]

use husky_salsa_log_utils::HasLogger;

use expect_test::expect;
use salsa::Db;

#[salsa::jar]
struct Jar(MyInput, result_depends_on_x, result_depends_on_y);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    x: u32,
    y: u32,
}

#[salsa::tracked(jar = Jar)]
fn result_depends_on_x(db: &Db, input: MyInput) -> u32 {
    db.push_log(format!("result_depends_on_x({:?})", input));
    input.x(db) + 1
}

#[salsa::tracked(jar = Jar)]
fn result_depends_on_y(db: &Db, input: MyInput) -> u32 {
    db.push_log(format!("result_depends_on_y({:?})", input));
    input.y(db) - 1
}

#[salsa::db(Jar)]
struct Database;

#[test]
fn execute() {
    // result_depends_on_x = x + 1
    // result_depends_on_y = y - 1
    let mut db = Database::default();

    let input = MyInput::new(&db, 22, 33, salsa::Durability::LOW);
    assert_eq!(result_depends_on_x(&db, input), 23);
    db.assert_logs(expect![[r#"
        [
            "result_depends_on_x(MyInput(Id { value: 1 }))",
        ]"#]]);

    assert_eq!(result_depends_on_y(&db, input), 32);
    db.assert_logs(expect![[r#"
        [
            "result_depends_on_y(MyInput(Id { value: 1 }))",
        ]"#]]);

    input.set_x(salsa::Durability::LOW, &mut db).to(23);
    // input x changes, so result depends on x needs to be recomputed;
    assert_eq!(result_depends_on_x(&db, input), 24);
    db.assert_logs(expect![[r#"
        [
            "result_depends_on_x(MyInput(Id { value: 1 }))",
        ]"#]]);

    // input y is the same, so result depends on y
    // does not need to be recomputed;
    assert_eq!(result_depends_on_y(&db, input), 32);
    db.assert_logs(expect!["[]"]);
}
