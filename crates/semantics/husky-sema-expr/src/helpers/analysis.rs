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
                | FugitiveKind::AliasType
                | FugitiveKind::Val
                | FugitiveKind::Formal => (),
            },
            SemaExprData::AssociatedItem {
                static_dispatch: StaticDispatch::AssociatedGn,
                ..
            }
            | SemaExprData::MethodGnCall { .. } => return true,
            _ => (),
        }
    }
    false
}
