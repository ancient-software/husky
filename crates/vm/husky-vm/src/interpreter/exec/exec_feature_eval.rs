use crate::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn exec_feature_eval(
        &mut self,
        feature_uid: EntityUid,
        mode: Mode,
        ins: &Instruction,
        ty: Term,
    ) -> __VMResult<()> {
        let ctx = self.opt_ctx.unwrap();
        let result = ctx.eval_feature_from_uid(feature_uid.raw());
        match mode {
            Mode::Fast | Mode::TrackMutation => (),
            Mode::TrackHistory => {
                self.history.write(
                    ins,
                    HistoryEntry::PureExpr {
                        result: result.clone(),
                        ty,
                    },
                );
            }
        }
        self.stack.push(result?);
        Ok(())
    }
}
