use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::*;
use syntax_types::{RoutineHead, RoutineKind};

impl EntityDefnKind {
    pub(crate) fn routine(
        db: &dyn EntityQueryGroup,
        routine_kind: &RoutineKind,
        routine_head: &RoutineHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnKind> {
        Ok(match routine_kind {
            RoutineKind::Test => todo!(),
            RoutineKind::Proc => {
                let stmts = parse_impr_stmts(
                    &routine_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnKind::Proc {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts,
                }
            }
            RoutineKind::Func => {
                let stmts = parse_decl_stmts(
                    &routine_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnKind::Func {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts,
                }
            }
            RoutineKind::Def => todo!(),
        })
    }
}
