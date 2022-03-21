use std::sync::Arc;

use crate::*;
use scope::{InputPlaceholder, RangedScope};
use syntax_types::*;
use vm::EagerContract;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn func_decl(mut self) -> AstResult<RoutineHead> {
        let funcname = get!(self, custom_ident);
        let space_params = self.placeholders()?;
        let input_contracts = self.func_input_placeholders()?;
        let output = self.func_output_type()?;
        Ok(RoutineHead {
            funcname,
            generics: space_params,
            input_placeholders: input_contracts,
            output,
        })
    }

    fn placeholders(&mut self) -> AstResult<Vec<GenericPlaceholder>> {
        if next_matches!(self, "<") {
            Ok(comma_list![self, placeholder!+, ">"])
        } else {
            Ok(Vec::new())
        }
    }

    fn placeholder(&mut self) -> AstResult<GenericPlaceholder> {
        let ident = get!(self, custom_ident);
        let mut traits = Vec::new();
        if next_matches!(self, ":") {
            traits.push(RangedScope {
                scope: get!(self, ty?),
                range: self.stream.pop_range(),
            });
            if next_matches!(self, "+") {
                todo!()
            }
        }
        Ok(GenericPlaceholder {
            ident,
            kind: GenericPlaceholderKind::Type { traits },
        })
    }

    fn func_input_placeholders(&mut self) -> AstResultArc<Vec<InputPlaceholder>> {
        no_look_pass!(self, "(");
        Ok(Arc::new(comma_list!(self, func_input_placeholder!, ")")))
    }

    fn func_input_placeholder(&mut self) -> AstResult<InputPlaceholder> {
        let ident = get!(self, custom_ident);
        no_look_pass!(self, ":");
        let ty = RangedScope {
            scope: get!(self, ty?),
            range: self.stream.pop_range(),
        };
        Ok(InputPlaceholder {
            ident,
            contract: EagerContract::Pure,
            ranged_ty: ty,
        })
    }

    fn func_output_type(&mut self) -> AstResult<RangedScope> {
        Ok(if next_matches!(self, "->") {
            RangedScope {
                scope: get!(self, ty?),
                range: self.stream.pop_range(),
            }
        } else {
            RangedScope {
                scope: ScopePtr::Builtin(BuiltinIdentifier::Void),
                range: self.stream.pop_range(),
            }
        })
    }
}
