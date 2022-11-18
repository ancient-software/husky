//! Test that if field X of an input changes but not field Y,
//! functions that depend on X re-execute, but those depending only on Y do not
//! compiles and executes successfully.
#![allow(dead_code)]

use husky_salsa_log_utils::{HasLogger, Logger};

use expect_test::expect;

#[timed_salsa::jar(db = Db)]
struct Jar(MyInput, result_depends_on_x, result_depends_on_y);

trait Db: timed_salsa::DbWithJar<Jar> + HasLogger {}

#[timed_salsa::input(jar = Jar)]
struct MyInput {
    x: u32,
    y: u32,
}

#[timed_salsa::tracked(jar = Jar)]
fn result_depends_on_x(db: &dyn Db, input: MyInput) -> u32 {
    db.push_log(format!("result_depends_on_x({:?})", input));
    input.x(db) + 1
}

#[timed_salsa::tracked(jar = Jar)]
fn result_depends_on_y(db: &dyn Db, input: MyInput) -> u32 {
    db.push_log(format!("result_depends_on_y({:?})", input));
    input.y(db) - 1
}

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: timed_salsa::Storage<Self>,
    logger: Logger,
}

impl timed_salsa::Database for Database {}

impl Db for Database {}

impl HasLogger for Database {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

#[test]
fn execute() {
    // result_depends_on_x = x + 1
    // result_depends_on_y = y - 1
    let mut db = Database::default();

    let input = MyInput::new(&db, 22, 33);
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

    input.set_x(&mut db).to(23);
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
