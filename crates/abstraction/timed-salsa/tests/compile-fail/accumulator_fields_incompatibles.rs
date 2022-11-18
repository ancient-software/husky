#[timed_salsa::jar(db = Db)]
struct Jar(AccTwoUnnamedFields, AccNamedField);

trait Db: timed_salsa::DbWithJar<Jar> {}

// accumulator with more than one unnamed fields
#[timed_salsa::accumulator(jar = Jar)]
struct AccTwoUnnamedFields(u32, u32);

// accumulator with named fields
#[timed_salsa::accumulator(jar = Jar)]
struct AccNamedField {
    field: u32,
}

fn main() {}
