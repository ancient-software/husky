use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureLazyStmt,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => self.feature_expr_figure(value),
            FeatureLazyStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureLazyStmtVariant::Return { ref result } => self.feature_expr_figure(result),
            FeatureLazyStmtVariant::ConditionFlow { ref branches } => todo!(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
            FeatureLazyStmtVariant::Require { ref condition, .. } => Ok(FigureCanvasData::void()),
        }
    }
}
