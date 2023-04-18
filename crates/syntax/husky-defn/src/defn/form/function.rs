use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FnDefn {
    #[id]
    pub path: FormPath,
    pub decl: FnDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn function_defn(db: &dyn DefnDb, decl: FnDecl) -> FnDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::Entity(path.into()),
        Some(decl.expr_region(db)),
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn {
            block: DefnBlock::Form { body, .. },
            ..
        } => body.map(|body| parser.parse_block_expr(body)),
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    FnDefn::new(db, path, decl, expr_region, body)
}
