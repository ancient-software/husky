mod expr;
mod var;

use std::{collections::HashMap, sync::Arc};

use ast::{AstText, RawExpr};
use builder::TySheetBuilder;
use expr::TySheetExprEntry;
use fold::FoldStorage;
use infer_error::ok_or;
use text::Row;
use var::TySheetVarEntry;
use vec_map::VecDict;
use word::CustomIdentifier;

use super::*;

pub(crate) fn ty_sheet(db: &dyn InferTySalsaQueryGroup, file: FilePtr) -> ScopeResultArc<TySheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = TySheetBuilder::new(db, ast_text.clone());
    ty_sheet_builder.infer_all(ast_text.folded_results.fold_iter(0));
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TySheet {
    pub ast_text: Arc<AstText>,
    pub(crate) exprs: HashMap<RawExprIdx, InferResult<EntityRoutePtr>>,
    pub(crate) variables: HashMap<(CustomIdentifier, Row), Option<EntityRoutePtr>>,
}

impl TySheet {
    pub(crate) fn new(ast_text: Arc<AstText>) -> Self {
        Self {
            exprs: Default::default(),
            variables: Default::default(),
            ast_text,
        }
    }

    pub(crate) fn expr_ty_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        self.exprs[&expr_idx].clone()
    }

    pub fn errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = Vec::new();
        for (_, result) in self.exprs.iter() {
            match result {
                Ok(_) => (),
                Err(error) => match error.kind {
                    InferErrorKind::Derived => (),
                    InferErrorKind::Original { .. } => errors.push(error),
                },
            }
        }
        errors
    }
}
