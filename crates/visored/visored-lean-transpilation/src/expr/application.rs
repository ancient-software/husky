use super::*;
use crate::dictionary::func_key::VdFuncKeyTranslation;
use either::*;
use lean_mir_expr::expr::application::LnMirFuncKey;
use smallvec::*;
use visored_mir_expr::expr::VdMirExprIdxRange;

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub(super) fn build_application(
        &mut self,
        expr: VdMirExprIdx,
        func: VdMirFunc,
        arguments: VdMirExprIdxRange,
    ) -> LnMirExprData {
        match func.key_or_expr(self.db()) {
            Left(func_key) => {
                let Some(translation) = self.dictionary().func_translation(func_key) else {
                    todo!("no translation for func key `{func_key:?}`")
                };
                match *translation {
                    VdFuncKeyTranslation::BinaryOprAsSeparator(func_key) => {
                        self.build_separated_list(expr, func_key, arguments)
                    }
                    // TODO: implement
                    VdFuncKeyTranslation::InSet => LnMirExprData::Sorry,
                }
            }
            Right(_) => todo!(),
        }
    }

    fn build_separated_list(
        &mut self,
        expr: VdMirExprIdx,
        func_key: LnMirFuncKey,
        arguments: VdMirExprIdxRange,
    ) -> LnMirExprData {
        debug_assert!(arguments.len() >= 2);
        let mut argument_iter = arguments.into_iter();
        let fst = self.build_expr(argument_iter.next().unwrap());
        let snd = self.build_expr(argument_iter.next().unwrap());
        let mut result = LnMirExprData::Application {
            function: func_key.into(),
            arguments: self.alloc_exprs([fst, snd]),
        };
        for argument in argument_iter {
            let argument = self.build_expr(argument);
            result = LnMirExprData::Application {
                function: func_key.into(),
                arguments: self.alloc_exprs([result, argument]),
            };
        }
        result
    }
}
