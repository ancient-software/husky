mod builder;
mod coersion;
pub mod destroyer;
pub mod eval;
pub mod expr;
pub mod jar;
pub mod pattern;
pub mod region;
pub mod stmt;
pub mod storage;
#[cfg(test)]
mod tests;
mod variable;
pub mod version_stamp;

use self::builder::VmirExprBuilder;
use self::jar::VmirJar as Jar;
#[cfg(test)]
use self::tests::*;
use husky_task::linktime::IsLinktime;
use husky_task_interface::IsLinkageImpl;

pub(crate) trait ToVmir<LinkageImpl: IsLinkageImpl>: Copy {
    type Output;

    fn to_vmir<Linktime>(self, builder: &mut VmirExprBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>;
}
