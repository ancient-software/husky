use husky_linkage::linkage::Linkage;
use husky_task_interface::IsLinkageImpl;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub trait IsLinktime: Sized + Send {
    type LinkageImpl: IsLinkageImpl;
    // linktime has the responsibility to guarantee that the linkage provided is up to date.
    fn linkage_impl(&self, linkage: Linkage, db: &::salsa::Db) -> Self::LinkageImpl;
    fn new_linktime(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self;
}

pub struct VirtualLinktime;

impl IsLinktime for VirtualLinktime {
    type LinkageImpl = Linkage;

    #[inline(always)]
    fn linkage_impl(&self, linkage: Linkage, _db: &salsa::Db) -> Self::LinkageImpl {
        linkage
    }

    fn new_linktime(_target_path: LinktimeTargetPath, _db: &salsa::Db) -> Self {
        VirtualLinktime
    }
}
