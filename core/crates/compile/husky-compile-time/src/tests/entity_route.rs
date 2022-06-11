use crate::*;
use entity_syntax::EntityLocus;
use thin_vec::{thin_vec, ThinVec};
use word::RootIdentifier;

#[test]
fn no_error_single_file() {
    let mut db = HuskyCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
struct A {}

main:
    let a = 1
"#
        .into(),
    );

    let main_file = db.intern_file("haha/main.hsk".into());
    let pack = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subscope_table = db.subroute_table(pack).ok().unwrap();
    should_eq!(subscope_table.entries.len(), 2);
    should_eq!(subscope_table.errors.len(), 0);
}

#[test]
fn no_error_many_files() {
    let mut db = HuskyCompileTime::default();
    db.set_live_file_text(
        "haha/main.hsk".into(),
        r#"
mod husky_lord
struct A {}

main:
    let a = 1
"#
        .into(),
    );
    db.set_live_file_text(
        "haha/husky_lord.hsk".into(),
        r#"
struct B {}
"#
        .into(),
    );

    let main_file = db.intern_file("haha/main.hsk".into());
    let husky_lord_file = db.intern_file("haha/husky_lord.hsk".into());
    let package = db.intern_entity_route(EntityRoute::package(
        main_file,
        db.intern_word("haha".into()).opt_custom().unwrap(),
    ));
    let subroute_table = db.subroute_table(package).ok().unwrap();
    let husky_lord_route =
        db.make_subroute(package, db.intern_word("husky_lord").custom(), thin_vec![]);
    should_eq!(
        db.entity_locus(husky_lord_route).unwrap(),
        EntityLocus::Module {
            file: husky_lord_file
        }
    );
    should_eq!(subroute_table.entries.len(), 3);
    should_eq!(subroute_table.errors.len(), 0);
}

#[test]
fn datasets() {
    let db = HuskyCompileTime::default();
    let dataset_scope = db.make_route(EntityRoutePtr::Root(RootIdentifier::Datasets), thin_vec![]);
    let synthetic_scope = db
        .subroute_result(
            dataset_scope,
            db.intern_word("synthetic").opt_custom().unwrap(),
            thin_vec![],
        )
        .unwrap();
    let synthetic_trivial_scope = db
        .subroute_result(
            synthetic_scope,
            db.intern_word("trivial").opt_custom().unwrap(),
            thin_vec![],
        )
        .unwrap();
    let _synthetic_trivial_real1d_scope = db
        .subroute_result(
            synthetic_trivial_scope,
            db.intern_word("real1d").opt_custom().unwrap(),
            thin_vec![],
        )
        .unwrap();
}
