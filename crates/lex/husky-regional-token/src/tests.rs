#[salsa::db(
    husky_coword::CowordJar,
    husky_vfs::VfsJar,
    husky_token_data::jar::TokenDataJar,
    husky_token::TokenJar,
    husky_term_prelude::TermPreludeJar,
    husky_entity_path::jar::EntityPathJar
)]
#[derive(Default)]
pub(crate) struct DB;
