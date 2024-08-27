//! Accumulate values from within a tracked function.
//! Then mutate the values so that the tracked function re-executes.
//! Check that we accumulate the appropriate, new values.

use expect_test::expect;

use salsa::*;
use test_log::test;

#[salsa::jar]
struct Jar(List, Integers, compute);

#[salsa::db(Jar)]
struct Database;

#[salsa::input(db = Db)]
struct List {
    value: u32,
    next: Option<List>,
}

#[salsa::accumulator]
struct Integers(u32);

#[salsa::tracked]
fn compute(db: &Db, input: List) {
    eprintln!(
        "{:?}(value={:?}, next={:?})",
        input,
        input.value(db),
        input.next(db)
    );
    let result = if let Some(next) = input.next(db) {
        let next_integers = compute::accumulated::<Integers>(db, next);
        eprintln!("{:?}", next_integers);
        let v = input.value(db) + next_integers.iter().sum::<u32>();
        eprintln!("input={:?} v={:?}", input.value(db), v);
        v
    } else {
        input.value(db)
    };
    Integers::push(db, result);
    eprintln!("pushed result {:?}", result);
}

#[test]
fn test1() {
    let mut db = Database::default();

    let l0 = List::new(&db, 1, None, salsa::Durability::LOW);
    let l1 = List::new(&db, 10, Some(l0), salsa::Durability::LOW);

    compute(&db, l1);
    expect![[r#"
        [
            11,
            1,
        ]
    "#]]
    .assert_debug_eq(&compute::accumulated::<Integers>(&db, l1));

    l0.set_value(salsa::Durability::LOW, &mut db).to(2);
    compute(&db, l1);
    expect![[r#"
        [
            12,
            2,
        ]
    "#]]
    .assert_debug_eq(&compute::accumulated::<Integers>(&db, l1));
}
