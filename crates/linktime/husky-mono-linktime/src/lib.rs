mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinktimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_devsoul::linktime::IsLinktime;
use husky_devsoul_interface::IsLinkageImpl;
use husky_linkage::linkage::Linkage;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use std::collections::HashMap;

// this will transpile everything compilable to Rust
pub struct MonoLinktime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    /* ad hoc pub*/ pub internal: std::sync::RwLock<MonoLinktimeInternal<LinkageImpl>>,
}

impl<LinkageImpl> IsLinktime for MonoLinktime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    type LinkageImpl = LinkageImpl;

    fn linkage_impl(&self, key: Linkage, db: &::salsa::Db) -> LinkageImpl {
        if let Some(linkage) = self
            .internal
            .read()
            .expect("todo")
            .get_linkage_impl(key, db)
        {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_impl_with_reload(key, db)
        }
    }

    fn new_linktime(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinktimeInternal::new(target_path, db)),
        }
    }
}
