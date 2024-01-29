#[salsa::jar]
struct Jar(MyInput, tracked_method_on_untracked_impl);

#[salsa::input]
struct MyInput {
    field: u32,
}

impl MyInput {
    #[salsa::tracked]
    fn tracked_method_on_untracked_impl(self, db: &Db) -> u32 {
        input.field(db)
    }
}

fn main() {}
