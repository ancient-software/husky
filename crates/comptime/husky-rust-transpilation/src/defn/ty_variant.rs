use super::*;
use husky_hir_decl::decl::{
    EnumTupleVariantField, EnumTupleVariantHirDecl, EnumUnitTypeVariantHirDecl,
};

impl TranspileToRustWith for TypeVariantHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

impl TranspileToRustWith for EnumUnitTypeVariantHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        self.path(db).ident(db).transpile_to_rust(builder)
    }
}

impl TranspileToRustWith for EnumTupleVariantHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.with_hir_eager_expr_region(self.hir_eager_expr_region(db), |builder| {
            self.path(db).ident(db).transpile_to_rust(builder);
            builder.bracketed_comma_list(RustBracket::Par, self.fields(db))
        })
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for EnumTupleVariantField {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.ty().transpile_to_rust(builder)
    }
}
