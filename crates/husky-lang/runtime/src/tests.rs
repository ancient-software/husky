use husky_lang_db::*;

use crate::session::Session;

#[test]
fn simple() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

dataset:
    synthetic::trivial::real1d::dataset1()

main:
    a = 1
    b = 1
    assert a == b
    a
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let package = db.package(main_file_id).unwrap();
    let sess = Session::new(&package);
}
