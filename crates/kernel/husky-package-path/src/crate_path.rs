use crate::*;
use husky_word::Identifier;

#[salsa::interned(jar = PackagePathJar)]
pub struct CratePath {
    pub package_path: PackagePath,
    pub crate_kind: CrateKind,
}

impl CratePath {
    pub fn relative_path(&self, db: &dyn PackagePathDb) -> std::borrow::Cow<'static, str> {
        match self.crate_kind(db) {
            CrateKind::Library => "src/lib.hsy".into(),
            CrateKind::Main => "src/main.hsy".into(),
            CrateKind::Binary(_ident) => todo!(),
            CrateKind::StandaloneTest(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CrateKind {
    Library,
    Main,
    Binary(Identifier),
    StandaloneTest(Identifier),
}
