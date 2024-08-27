//! Test that the `constructor` macro overrides
//! the `new` method's name and `get` and `set`
//! change the name of the getter and setter of the fields.
#![allow(warnings)]
use salsa::*;
use std::fmt::Display;

#[salsa::jar]
struct Jar(MyInput, MyInterned, MyTracked);

#[salsa::input(db = Db, jar = Jar, constructor = from_string)]
struct MyInput {
    #[get(text)]
    #[set(set_text)]
    field: String,
}

impl MyInput {
    pub fn new(db: &mut Db, s: impl Display) -> MyInput {
        MyInput::from_string(db, s.to_string(), salsa::Durability::LOW)
    }

    pub fn field(self, db: &Db) -> String {
        self.text(db)
    }

    pub fn set_field(self, db: &mut Db, id: String) {
        self.set_text(salsa::Durability::LOW, db).to(id);
    }
}

#[salsa::interned(db = Db, constructor = from_string)]
struct MyInterned {
    #[get(text)]
    #[return_ref]
    field: String,
}

impl MyInterned {
    pub fn new(db: &Db, s: impl Display) -> MyInterned {
        MyInterned::from_string(db, s.to_string())
    }

    pub fn field(self, db: &Db) -> &str {
        &self.text(db)
    }
}

#[salsa::tracked(db = Db, constructor = from_string)]
struct MyTracked {
    #[get(text)]
    field: String,
}

impl MyTracked {
    pub fn new(db: &Db, s: impl Display) -> MyTracked {
        MyTracked::from_string(db, s.to_string())
    }

    pub fn field(self, db: &Db) -> String {
        self.text(db)
    }
}

#[test]
fn execute() {
    #[salsa::db(Jar)]
    #[derive(Default)]
    struct Database;

    let mut db = Database::default();
}
