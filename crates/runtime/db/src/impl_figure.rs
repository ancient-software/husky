use crate::*;
use compile_time_db::*;
use feature::{FeatureExpr, FeatureExprKind, FeatureStmt, FeatureStmtKind};

impl HuskyLangRuntime {
    pub fn figure(&self, trace_id: TraceId, focus: &Focus) -> FigureProps {
        let trace = self.trace(trace_id);
        match trace.kind {
            TraceKind::Main(_) => FigureProps::Blank,
            TraceKind::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt, focus),
            TraceKind::FeatureBranch(_) => FigureProps::Blank,
            TraceKind::FeatureExpr(ref expr) => self.feature_expr_figure(expr, focus),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt {
                ref stmt,
                ref history,
            } => todo!(),
            TraceKind::ImprStmt {
                ref stmt,
                ref history,
            } => todo!(),
            TraceKind::LoopFrame { .. } => todo!(),
            TraceKind::EagerExpr {
                ref expr,
                ref history,
            } => todo!(),
            TraceKind::CallHead {
                ref entity,
                ref tokens,
            } => todo!(),
        }
    }

    fn feature_stmt_figure(&self, stmt: &FeatureStmt, focus: &Focus) -> FigureProps {
        match stmt.kind {
            FeatureStmtKind::Init { varname, ref value } => self.feature_expr_figure(value, focus),
            FeatureStmtKind::Assert { .. } => FigureProps::Blank,
            FeatureStmtKind::Return { ref result } => self.feature_expr_figure(result, focus),
            FeatureStmtKind::BranchGroup { kind, ref branches } => FigureProps::Blank,
        }
    }

    fn feature_expr_figure(&self, expr: &FeatureExpr, focus: &Focus) -> FigureProps {
        match expr.kind {
            FeatureExprKind::PrimitiveLiteral(_) => FigureProps::Blank,
            FeatureExprKind::EnumLiteral { ref value, uid } => todo!(),
            FeatureExprKind::PrimitiveBinaryOpr {
                opr,
                ref lopd,
                ref ropd,
            } => todo!(),
            FeatureExprKind::Variable { varname, ref value } => todo!(),
            FeatureExprKind::This { ref repr } => todo!(),
            FeatureExprKind::FuncCall {
                func_ranged_scope,
                ref inputs,
                uid,
                callee_file,
                ref input_placeholders,
                compiled,
                ref instruction_sheet,
                ref stmts,
            } => todo!(),
            FeatureExprKind::ProcCall {
                proc_ranged_scope,
                ref inputs,
                uid,
                callee_file,
                ref input_placeholders,
                compiled,
                ref instruction_sheet,
                ref stmts,
            } => todo!(),
            FeatureExprKind::StructMembVarAccess {
                ref this,
                memb_ident,
                contract,
                opt_compiled,
            } => todo!(),
            FeatureExprKind::RecordMembAccess {
                ref this,
                memb_ident,
                ref repr,
            } => todo!(),
            FeatureExprKind::MembFuncCall {
                memb_ident,
                ref opds,
                ref instruction_sheet,
                compiled,
                ref stmts,
            } => todo!(),
            FeatureExprKind::MembProcCall {
                memb_ident,
                ref opds,
                ref instruction_sheet,
                compiled,
                ref stmts,
            } => todo!(),
            FeatureExprKind::MembPattCall {
                memb_ident,
                ref opds,
                ref instruction_sheet,
                ref stmts,
            } => todo!(),
            FeatureExprKind::FeatureBlock { scope, ref block } => todo!(),
            FeatureExprKind::GlobalInput => match focus.opt_input_id {
                Some(input_id) => {
                    let global_input: Arc<dyn AnyValueDyn<'static> + 'static> = {
                        let session: &mut Session = &mut self.session.lock().unwrap();
                        let dev_division = &mut session.dev;
                        dev_division.loader.load(input_id).input.clone()
                    };
                    let global_input_ty = self
                        .compile_time
                        .global_input_ty(self.package_main)
                        .unwrap();
                    let visualizer = self.visualizer(self.version(), global_input_ty);
                    let global_input_ref = &*global_input;
                    let visual_props = visualizer.visualize(global_input_ref);
                    FigureProps::new_specific(visual_props)
                }
                None => todo!(),
            },
            FeatureExprKind::ClassCall {
                ty,
                ref entity,
                ref opds,
            } => todo!(),
        }
    }
}
