use husky_fly_term::dispatch::StaticDispatch;

use super::*;

#[salsa::tracked(jar = SemaExprJar)]
pub fn sema_expr_region_contains_gn(db: &::salsa::Db, sema_expr_region: SemaExprRegion) -> bool {
    for sema_expr_entry in sema_expr_region.data(db).sema_expr_arena().iter() {
        match sema_expr_entry.data() {
            SemaExprData::PrincipalEntityPath {
                path: PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)),
                ..
            } => match path.fugitive_kind(db) {
                FugitiveKind::FunctionGn => return true,
                FugitiveKind::FunctionFn
                | FugitiveKind::TypeAlias
                | FugitiveKind::Val
                | FugitiveKind::Formal
                | FugitiveKind::Const => (),
            },
            SemaExprData::AssocItem {
                static_dispatch: StaticDispatch::AssocGn,
                ..
            }
            | SemaExprData::MethodGnCall { .. } => return true,
            _ => (),
        }
    }
    false
}
