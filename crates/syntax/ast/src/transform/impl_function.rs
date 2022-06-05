use crate::*;
use text::TextRanged;
use token::*;
use token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_function_defn_head(
        &mut self,
        paradigm: Paradigm,
        token_group: &[Token],
    ) -> AstResult<AstVariant> {
        let tokens = trim_colon!(token_group; keyword, colon);
        let head = match paradigm {
            Paradigm::EagerProcedural => {
                self.context
                    .set(AstContext::Stmt(Paradigm::EagerProcedural));
                self.parse_atoms(tokens, |parser| {
                    parser.call_defn_head(None, Paradigm::EagerProcedural)
                })?
            }
            Paradigm::EagerFunctional => {
                self.context
                    .set(AstContext::Stmt(Paradigm::EagerFunctional));
                self.parse_atoms(tokens, |parser| {
                    parser.call_defn_head(None, Paradigm::EagerFunctional)
                })?
            }
            Paradigm::LazyFunctional => todo!(),
        };
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstVariant::CallFormDefnHead(head))
    }
}
