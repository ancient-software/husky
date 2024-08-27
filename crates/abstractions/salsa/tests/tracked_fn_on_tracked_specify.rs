//! Test that a `tracked` fn on a `salsa::input`
//! compiles and executes successfully.
#![allow(warnings)]

use salsa::Db;

#[salsa::jar]
struct Jar(MyInput, MyTracked, tracked_fn, tracked_fn_extra);

#[salsa::input(db = Db, jar = Jar)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(db = Db, jar = Jar)]
struct MyTracked {
    field: u32,
}

#[salsa::tracked(jar = Jar)]
fn tracked_fn(db: &Db, input: MyInput) -> MyTracked {
    let t = MyTracked::new(db, input.field(db) * 2);
    if input.field(db) != 0 {
        tracked_fn_extra::specify(db, t, 2222);
    }
    t
}

#[salsa::tracked(jar = Jar, specify)]
fn tracked_fn_extra(_db: &Db, _input: MyTracked) -> u32 {
    0
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
fn execute_when_specified() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 22, salsa::Durability::LOW);
    let tracked = tracked_fn(&db, input);
    assert_eq!(tracked.field(&db), 44);
    assert_eq!(tracked_fn_extra(&db, tracked), 2222);
}

#[test]
fn execute_when_not_specified() {
    let mut db = Database::default();
    let input = MyInput::new(&db, 0, salsa::Durability::LOW);
    let tracked = tracked_fn(&db, input);
    assert_eq!(tracked.field(&db), 0);
    assert_eq!(tracked_fn_extra(&db, tracked), 0);
}
