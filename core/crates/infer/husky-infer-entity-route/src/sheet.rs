mod expr;
mod var;

use std::{collections::HashMap, sync::Arc};

use arena::map::ArenaMap;
use builder::EntityRouteSheetBuilder;
use dev_utils::dev_src;
use fold::FoldableStorage;
use husky_ast::{AstText, RawExpr};
use husky_text::{Row, TextRange};
use infer_decl::MemberIdx;
use word::CustomIdentifier;

use super::*;

pub(crate) fn entity_route_sheet(
    db: &dyn InferEntityRouteQueryGroup,
    file: FilePtr,
) -> EntitySyntaxResultArc<EntityRouteSheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = EntityRouteSheetBuilder::new(db, &ast_text.arena, ast_text.clone());
    ty_sheet_builder.infer_all(ast_text.folded_results.iter());
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityRouteSheet {
    pub ast_text: Arc<AstText>,
    pub(crate) expr_tys: RawExprMap<InferResult<EntityRoutePtr>>,
    pub(crate) call_routes: RawExprMap<InferResult<EntityRoutePtr>>,
    pub(crate) variable_tys: HashMap<(CustomIdentifier, TextRange), EntityRoutePtr>,
    pub(crate) extra_errors: Vec<InferError>,
}

impl EntityRouteSheet {
    fn expr(&self, idx: RawExprIdx) -> &RawExpr {
        &self.ast_text.arena[idx]
    }

    pub(crate) fn new(ast_text: Arc<AstText>, extra_errors: Vec<InferError>) -> Self {
        Self {
            expr_tys: ArenaMap::new(&ast_text.arena),
            call_routes: ArenaMap::new(&ast_text.arena),
            variable_tys: Default::default(),
            ast_text,
            extra_errors,
        }
    }

    pub fn expr_ty_result(&self, idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        if let Some(ref expr_ty) = self.expr_tys.get(idx) {
            match expr_ty {
                Ok(ty) => Ok(*ty),
                Err(e) => Err(e.derived()),
            }
        } else {
            Err(InferError {
                variant: InferErrorVariant::Derived {
                    message: "failed to infer expr ty".into(),
                },
                dev_src: dev_src!(),
            })
        }
    }

    pub fn opt_call_route(&self, idx: RawExprIdx) -> Option<InferResult<EntityRoutePtr>> {
        Some(match self.call_routes.get(idx)? {
            Ok(call_route) => Ok(*call_route),
            Err(e) => Err(e.derived()),
        })
    }

    pub fn original_errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = self
            .extra_errors
            .iter()
            .filter_map(|e| match e.variant {
                InferErrorVariant::Derived { .. } => None,
                InferErrorVariant::Original { .. } => Some(e),
            })
            .collect();
        for result in self.expr_tys.values() {
            match result {
                Ok(_) => (),
                Err(error) => match error.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(error),
                },
            }
        }

        errors
    }
}
