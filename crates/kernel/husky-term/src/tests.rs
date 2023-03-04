use crate::TermJar;
use husky_entity_path::EntityPathJar;
use husky_precise_term::PreciseTermJar;
use husky_raw_term::RawTermJar;
use husky_valid_term::ValidTermJar;
use husky_vfs::VfsJar;
use husky_word::WordJar;

#[salsa::db(
    EntityPathJar,
    VfsJar,
    WordJar,
    RawTermJar,
    PreciseTermJar,
    ValidTermJar,
    TermJar
)]
#[derive(Default)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
}

impl salsa::Database for TermTestsDb {}
