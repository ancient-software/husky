use super::*;
use husky_hir_decl::decl::TraitMethodFnHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitMethodFnHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TraitMethodFnHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TraitMethodFnHirDefn) -> Self {
        AssocItemHirDefn::TraitItem(hir_defn.into())
    }
}

impl From<TraitMethodFnHirDefn> for HirDefn {
    fn from(hir_defn: TraitMethodFnHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitMethodFnHirDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TraitItemPath,
        _hir_decl: TraitMethodFnHirDecl,
    ) -> Self {
        todo!()
        // let TraitItemHirNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TraitMethodFnHirDefn::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // ))
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        self.eager_body_with_hir_eager_expr_region(db)
            .map(|(_, region)| region)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_method_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_method_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_method_fn_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitMethodFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_item_path(hir_decl.path(db).trai_path(db));
    for param in hir_decl.parenate_parameters(db).iter() {
        match *param {
            HirEagerParenateParameter::Simple { ty, .. } => builder.add_hir_ty(ty),
            HirEagerParenateParameter::Keyed => todo!(),
            HirEagerParenateParameter::Variadic => todo!(),
        }
    }
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_method_fn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitMethodFnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
