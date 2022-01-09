use crate::*;

#[test]
fn no_error_single_file() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A:
    a: i32

main:
    let a = 1
    let b = 1
    assert a == c
"#
        .into(),
    );

    let main_file_id = db.file_id("haha/main.hsk".into());
    let ast_text = db.ast_text(main_file_id);
    // ep!(expr_text);
}
