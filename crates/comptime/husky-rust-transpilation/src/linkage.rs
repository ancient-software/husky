use crate::*;
use husky_linkage::linkage::package_linkages;
use salsa::DebugWithDb;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(db, package_path.toolchain(db), None);
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    for linkage in package_linkages(db, package_path) {
        builder.on_fresh_paragraph(|builder| {
            builder.comment(&format!("todo: {:#?}", linkage.debug(db)))
        })
    }
    builder_base.finish()
}
