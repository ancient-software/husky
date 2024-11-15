use super::*;
use crate::builder::VdLeanTranspilationBuilder;
use dictionary::VdLeanDictionary;
use either::*;
use husky_tree_utils::display::DisplayTree;
use latex_prelude::mode::LxMode;
use lean_mir_expr::{
    expr::{LnMirExprArena, LnMirExprIdx},
    helpers::{
        fmt::{LnMirExprFormatter, LnMirExprFormatterConfig},
        show::display_tree::LnMirExprDisplayTreeBuilder,
    },
    item_defn::{LnItemDefnArena, LnItemDefnIdxRange},
    stmt::{LnMirStmtArena, LnMirStmtIdxRange},
    tactic::LnMirTacticArena,
};
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_mir_expr::{
    expr::VdMirExprIdx, stmt::VdMirStmtIdxRange, test_helpers::example::VdMirExprExample,
};

pub struct VdLeanTranspilationExample {
    expr_arena: LnMirExprArena,
    stmt_arena: LnMirStmtArena,
    tactic_arena: LnMirTacticArena,
    defn_arena: LnItemDefnArena,
    result: Either<LnMirExprIdx, LnItemDefnIdxRange>,
}

impl VdLeanTranspilationExample {
    pub fn new(
        input: &str,
        root_mode: LxMode,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        db: &::salsa::Db,
    ) -> Self {
        let VdMirExprExample {
            expr_arena: vd_mir_expr_arena,
            stmt_arena: vd_mir_stmt_arena,
            symbol_local_defn_storage: vd_mir_symbol_local_defn_storage,
            result,
        } = VdMirExprExample::new(input, root_mode, &[], &[], db);
        let dictionary = &VdLeanDictionary::new_standard(db);
        let mut builder = VdLeanTranspilationBuilder::new(
            db,
            vd_mir_expr_arena.as_arena_ref(),
            vd_mir_stmt_arena.as_arena_ref(),
            &vd_mir_symbol_local_defn_storage,
            dictionary,
        );
        let result = match result {
            Left(expr) => Left(expr.to_lean(&mut builder)),
            Right(stmts) => Right(stmts.to_lean(&mut builder)),
        };
        let (expr_arena, stmt_arena, tactic_arena, defn_arena) = builder.finish();
        Self {
            expr_arena,
            stmt_arena,
            tactic_arena,
            defn_arena,
            result,
        }
    }

    pub fn show_display_tree(&self, db: &::salsa::Db) -> String {
        self.display_tree(db).show(&Default::default())
    }

    fn display_tree(&self, db: &::salsa::Db) -> DisplayTree {
        let builder = LnMirExprDisplayTreeBuilder::new(
            db,
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.tactic_arena.as_arena_ref(),
            self.defn_arena.as_arena_ref(),
        );
        match self.result {
            Left(expr) => builder.render_expr(expr),
            Right(defns) => builder.render_defns_together(defns),
        }
    }

    pub fn show_fmt(&self, db: &::salsa::Db) -> String {
        let fmt_config = Default::default();
        let mut formatter = self.formatter(db, &fmt_config);
        match self.result {
            Left(expr) => formatter.format_expr_ext(expr),
            Right(defns) => formatter.format_defns(defns),
        }
        formatter.finish()
    }

    fn formatter<'a>(
        &'a self,
        db: &'a ::salsa::Db,
        config: &'a LnMirExprFormatterConfig,
    ) -> LnMirExprFormatter<'a> {
        LnMirExprFormatter::new(
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.tactic_arena.as_arena_ref(),
            self.defn_arena.as_arena_ref(),
            config,
            db,
        )
    }
}
