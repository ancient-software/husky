use salsa::*;
use test_log::test;

#[salsa::jar]
struct Jar(MyInput, MyTracked, tracked_fn);

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
    MyTracked::new(db, input.field(db) / 2)
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
#[should_panic(expected = "`execute` method for field")]
fn execute() {
    let mut db = Database::default();

    let input = MyInput::new(&db, 22, salsa::Durability::LOW);
    let tracked = tracked_fn(&db, input);

    // modify the input and change the revision
    input.set_field(salsa::Durability::LOW, &mut db).to(24);

    // panic when reading fields of tracked structs from older revisions
    tracked.field(&db);
}
