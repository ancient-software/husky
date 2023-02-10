use husky_token::FloatLiteral;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr(
        &mut self,
        expr_idx: ExprIdx,
        expectation: LocalTermExpectation,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc_expr(expr_idx, expectation);
        let (ty, opt_expectation) = match ty_result {
            Ok(ty) => (
                Some(ty),
                self.add_expectation_rule(expr_idx, ty, expectation),
            ),
            Err(_) => (None, Default::default()),
        };
        self.save_expr(expr_idx, ExprTypeInfo::new(ty_result, opt_expectation));
        ty
    }

    fn infer_new_expr_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expectation: LocalTermExpectation,
    ) -> Option<ReducedTerm> {
        match self.infer_new_expr(expr_idx, expectation)? {
            LocalTerm::Resolved(lopd_ty) => Some(lopd_ty),
            LocalTerm::Unresolved(lopd_ty) => self.resolve_term(lopd_ty),
        }
    }

    fn save_expr(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr(
        &mut self,
        expr_idx: ExprIdx,
        expectation: LocalTermExpectation,
    ) -> ExprTypeResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => {
                self.calc_literal(expr_idx, literal_token_idx, expectation)
            }
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => match self.db.entity_ty(entity_path) {
                    Ok(ty) => Ok(ty.into()),
                    Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                },
                None => todo!(),
            },
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                Some(ty) => Ok((*ty).into()),
                None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
            },
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => self
                .current_symbol_tys
                .get(current_symbol_idx)
                .copied()
                .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into()),
            Expr::FrameVarDecl {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd, opr, ropd, ..
            } => self.calc_binary(lopd, ropd),
            Expr::Be {
                src, ref target, ..
            } => todo!(),
            Expr::PrefixOpn { opr, opd, .. } => self.calc_prefix(opd, opr),
            Expr::SuffixOpn {
                opd, punctuation, ..
            } => todo!(),
            Expr::ApplicationOrFunctionCall {
                function, argument, ..
            } => {
                let function_ty = self.infer_new_expr(function, LocalTermExpectation::None);
                match function_ty {
                    Some(function_ty) => todo!(),
                    None => {
                        self.infer_new_expr(argument, LocalTermExpectation::None);
                        Err(DerivedExprTypeError::FunctionTypeNotInferredInApplicationOrFunctionCall.into())
                    }
                }
            }
            Expr::FunctionCall {
                function,
                ref implicit_arguments,
                arguments,
                ..
            } => {
                let function_ty = self.infer_new_expr(function, LocalTermExpectation::None);
                self.calc_call_expr(None, function_ty, implicit_arguments.as_ref(), arguments)
            }
            Expr::Field {
                owner, ident_token, ..
            } => {
                if let Some(owner_ty) =
                    self.infer_new_expr_resolved(owner, LocalTermExpectation::None)
                {
                    let field_ty = self.db.field_ty(owner_ty, ident_token.ident());
                    match field_ty {
                        Ok(_) => todo!(),
                        Err(e) => Err(e.into()),
                    }
                } else {
                    Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
                }
            }
            Expr::MethodCall {
                self_expr,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => {
                let Some(self_expr_ty) =
                    self.infer_new_expr_resolved( self_expr, LocalTermExpectation::None)
                    else {
                        if let Some(implicit_arguments) = implicit_arguments {
                            todo!()
                        }
                        for argument in nonself_arguments {
                            todo!()
                        }
                        return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
                    };
                let method_ty = match self.db.ty_method_ty(self_expr_ty, ident_token.ident()) {
                    Ok(_) => todo!(),
                    Err(e) => return Err(e.into()),
                };
                self.calc_call_expr(
                    Some(self_expr_ty),
                    method_ty,
                    implicit_arguments.as_ref(),
                    nonself_arguments,
                )
            }
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self.calc_application(function, argument),
            Expr::Bracketed { item, .. } => self
                .infer_new_expr(item, expectation)
                .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::NewBoxList { caller, items, .. } => self.calc_new_box_list(expr_idx, items),
            Expr::BoxColon { caller, .. } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_block(stmts, expectation)
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }

    fn calc_new_box_list(
        &mut self,
        expr_idx: ExprIdx,
        items: ExprIdxRange,
    ) -> Result<LocalTerm, ExprTypeError> {
        let element_ty = self.new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ImplicitType);
        for item in items {
            self.infer_new_expr(
                item,
                LocalTermExpectation::ImplicitlyConvertibleTo { term: element_ty },
            );
        }
        Ok(self
            .intern_unresolved_term(UnresolvedTerm::TypeApplication {
                ty: self.entity_path_menu.list_ty(),
                arguments: vec![element_ty],
            })
            .into())
    }

    fn calc_call_expr(
        &mut self,
        self_ty: Option<ReducedTerm>,
        callable_ty: Option<LocalTerm>,
        implicit_arguments: Option<&ImplicitArgumentList>,
        arguments: ExprIdxRange,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(callable_ty) = callable_ty
            else {
                if let Some(implicit_arguments) = implicit_arguments{
                    for argument in implicit_arguments.arguments() {
                        self.infer_new_expr(argument, LocalTermExpectation::None);
                    }
                }
                for argument in arguments {
                    self.infer_new_expr(argument, LocalTermExpectation::None);
                }
                return Err(DerivedExprTypeError::CallableTypeError.into())
            };
        todo!()
    }

    fn calc_binary(&mut self, lopd: ExprIdx, ropd: ExprIdx) -> ExprTypeResult<LocalTerm> {
        let Some(lopd_ty) = self.infer_new_expr_resolved(lopd, LocalTermExpectation::None)
            else {
                return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
            };
        let Some(ropd_ty) = self.infer_new_expr_resolved(ropd, LocalTermExpectation::None)
            else {
                return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
            };
        todo!()
    }

    fn calc_prefix(&mut self, opd: ExprIdx, opr: PrefixOpr) -> ExprTypeResult<LocalTerm> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self.infer_new_expr(opd, LocalTermExpectation::None);
                match opd_ty {
                    Some(opd_ty) => match opd_ty {
                        LocalTerm::Resolved(_) => todo!(),
                        LocalTerm::Unresolved(unresolved_term) => {
                            match self.unresolved_term_table[unresolved_term].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    match implicit_symbol.variant() {
                                        ImplicitSymbolVariant::Lifetime => todo!(),
                                        ImplicitSymbolVariant::UnspecifiedIntegerType
                                        | ImplicitSymbolVariant::UnspecifiedFloatType => Ok(opd_ty),
                                        ImplicitSymbolVariant::ImplicitType => todo!(),
                                    }
                                }
                                UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                            }
                        }
                    },
                    None => Err(DerivedExprTypeError::PrefixOperandTypeNotInferred.into()),
                }
            }
            PrefixOpr::Not => {
                let _opd_ty = self.infer_new_expr(opd, LocalTermExpectation::AsBool);
                // here we differs from Rust, but agrees with C
                Ok(self.reduced_term_menu.bool().into())
            }
            PrefixOpr::BitNot => todo!(),
            PrefixOpr::Ref => {
                let opd_ty = self.infer_new_expr(opd, LocalTermExpectation::None);
                // Should consider more cases, could also be taking references
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                let opd_ty = self.infer_new_expr(opd, LocalTermExpectation::Type);
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
        }
    }

    fn calc_application(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let function_expr = &self[function];
        match function_expr {
            Expr::NewBoxList {
                caller: None,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => {
                match items.len() {
                    0 => {
                        let argument_ty = self.infer_new_expr(argument, LocalTermExpectation::None);
                        // check this is type
                        argument_ty
                            .ok_or(DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into())
                    }
                    1 => {
                        let arg0_ty =
                            self.infer_new_expr(items.start(), LocalTermExpectation::None);
                        match arg0_ty {
                            Some(_) => todo!(),
                            None => {
                                Err(DerivedExprTypeError::BoxListApplicationFirstArgumentError
                                    .into())
                            }
                        }
                    }
                    n => {
                        todo!()
                    }
                }
            }
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            _ => {
                let function_ty = self.infer_new_expr(function, LocalTermExpectation::None);
                todo!()
            }
        }
    }

    fn calc_literal(
        &mut self,
        expr_idx: ExprIdx,
        literal_token_idx: TokenIdx,
        expectation: LocalTermExpectation,
    ) -> Result<LocalTerm, ExprTypeError> {
        let literal_token = self.token_sheet_data[literal_token_idx];
        match literal_token {
            Token::Literal(literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => Ok(self.reduced_term_menu.static_str_ref().into()),
                Literal::Integer(integer_literal) => match integer_literal {
                    IntegerLiteral::Unspecified => match expectation.term() {
                        // MOM
                        Some(term) if term == self.reduced_term_menu.i32() => todo!(),
                        _ => Ok(self
                            .new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedIntegerType,
                            )
                            .into()),
                    },
                    IntegerLiteral::I8(_) => todo!(),
                    IntegerLiteral::I16(_) => todo!(),
                    IntegerLiteral::I32(_) => Ok(self.reduced_term_menu.i32().into()),
                    IntegerLiteral::I64(_) => todo!(),
                    IntegerLiteral::I128(_) => todo!(),
                    IntegerLiteral::ISize(_) => todo!(),
                    IntegerLiteral::R8(_) => todo!(),
                    IntegerLiteral::R16(_) => todo!(),
                    IntegerLiteral::R32(_) => todo!(),
                    IntegerLiteral::R64(_) => todo!(),
                    IntegerLiteral::R128(_) => todo!(),
                    IntegerLiteral::RSize(_) => todo!(),
                    IntegerLiteral::U8(_) => todo!(),
                    IntegerLiteral::U16(_) => todo!(),
                    IntegerLiteral::U32(_) => todo!(),
                    IntegerLiteral::U64(_) => todo!(),
                    IntegerLiteral::U128(_) => todo!(),
                    IntegerLiteral::USize(_) => todo!(),
                },
                Literal::Float(float_literal) => match float_literal {
                    FloatLiteral::Unspecified => match expectation {
                        LocalTermExpectation::None => {
                            let ty = self.new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedFloatType,
                            );
                            Ok(ty.into())
                        }
                        LocalTermExpectation::Type => todo!(),
                        LocalTermExpectation::AsBool => todo!(),
                        LocalTermExpectation::Return { ty } => todo!(),
                        LocalTermExpectation::ImplicitlyConvertibleTo { term: ty } => todo!(),
                    },
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.reduced_term_menu.bool().into()),
            },
            _ => unreachable!(),
        }
    }
}
