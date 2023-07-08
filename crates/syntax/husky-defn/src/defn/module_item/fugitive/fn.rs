use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct FnNodeDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub node_decl: FnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl FnNodeDefn {
    pub(super) fn new(db: &dyn DefnDb, node_path: FugitiveNodePath, node_decl: FnNodeDecl) -> Self {
        let node_path = node_decl.node_path(db);
        let mut parser = expr_parser(
            db,
            node_path,
            node_decl.expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::Fugitive { body, .. },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        FnNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct FnDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: FnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl FnDefn {
    pub(super) fn new(db: &dyn DefnDb, path: FugitivePath, decl: FnDecl) -> Self {
        let FugitiveNodeDefn::Fn(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        FnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        )
    }
}
