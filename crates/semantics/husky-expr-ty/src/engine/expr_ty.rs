mod associated_item;
mod binary;
mod box_list;
mod current_symbol;
mod field;
mod function_application;
mod function_call;
mod index_or_compose_with_list;
mod literal;
mod method;
mod prefix;
mod ritchie_call_ty;
mod suffix;
mod utils;

pub(crate) use self::suffix::Unveiler;

use super::*;
use husky_opn_syntax::*;

pub(crate) enum ExprTypeResolveProgress<E: ExpectFluffyTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
    ) -> Option<FluffyTerm> {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        self.expr_ty_infos[expr_idx].ty().ok()
    }

    /// infer the type of a new expression but don't need the result for now
    pub(super) fn infer_new_expr_ty_discarded<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
    ) {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_for_outcome<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
    ) -> Option<E::Outcome>
    where
        E::Outcome: Clone,
    {
        let expectation_idx = self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation);
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db(), FluffyTermResolveLevel::Weak);
        let outcome = match expectation_idx.into_option() {
            Some(expectation_idx) => self.fluffy_term_region[expectation_idx]
                .resolve_progress()
                .outcome::<E>()
                .cloned(),
            None => None,
        };
        outcome
    }

    #[inline(always)]
    fn infer_new_expr_ty_aux<E: ExpectFluffyTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
    ) -> OptionFluffyTermExpectationIdx {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation);
        let expectation_idx = match ty_result {
            Ok((_, Ok(ty))) => self.fluffy_term_region.add_expectation(
                ExpectationSource::new_expr(expr_idx),
                ty,
                expr_ty_expectation,
            ),
            _ => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.fluffy_term_region
            .resolve_as_much_as_possible(self.db(), FluffyTermResolveLevel::Weak);
        expectation_idx
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx, _) => Ok((
                ExprDisambiguation::Trivial,
                self.calc_literal_expr_ty(expr_idx, literal_token_idx, expr_ty_expectation),
            )),
            Expr::PrincipalEntityPath {
                entity_path_expr,
                opt_path: path,
            } => self.calc_principal_entity_path_expr_ty(path, expr_ty_expectation),
            Expr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => self.calc_scope_resolution_ty(expr_idx, parent_expr_idx, ident_token),
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                match self
                    .symbol_tys
                    .inherited_symbol_map()
                    .get(inherited_symbol_idx)
                {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
                },
            )),
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.get_current_symbol_ty(expr_idx, current_symbol_idx),
            )),
            Expr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => Ok((
                ExprDisambiguation::Trivial,
                match self.self_ty {
                    Some(self_ty) => match self_ty.ty_unchecked(self.db)? {
                        Left(self_ty_ty) => Ok(self_ty_ty.into()),
                        Right(_) => unreachable!(),
                    }, // todo: impl binding
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            Expr::SelfValue(_) => Ok((
                ExprDisambiguation::Trivial,
                match self.self_ty {
                    Some(self_ty) => Ok(self_ty.into()), // todo: impl binding
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            Expr::Binary {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd),
            )),
            Expr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectAnyOriginal) {
                    Some(src_ty) => match target {
                        Ok(target) => self.infer_pattern_and_symbols_ty(
                            target.pattern_expr(),
                            src_ty,
                            target.variables(),
                        ),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            Expr::Prefix { opr, opd, .. } => self.calc_prefix_expr_ty(
                expr_idx,
                opr,
                opd,
                expr_ty_expectation.final_destination(self),
            ),
            Expr::Suffix { opd, opr, .. } => self.calc_suffix_expr_ty(
                expr_idx,
                opd,
                opr,
                expr_ty_expectation.final_destination(self),
            ),
            Expr::FunctionApplicationOrCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_function_application_or_call_expr_ty(
                expr_idx,
                function,
                expr_ty_expectation,
                implicit_arguments.as_ref(),
                items,
            ),
            Expr::FunctionCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_function_call_expr_ty(
                expr_idx,
                function,
                expr_ty_expectation.final_destination(self),
                implicit_arguments.as_ref(),
                items,
            ),
            Expr::Field {
                owner, ident_token, ..
            } => self.calc_field_expr_ty(owner, ident_token),
            Expr::MethodApplicationOrCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_method_application_or_call_ty(
                expr_idx,
                self_argument,
                ident_token,
                implicit_arguments.as_ref(),
                items,
            ),
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::ExplicitApplication {
                function_expr_idx,
                argument_expr_idx,
            } => self.calc_explicit_application(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                expr_ty_expectation.final_destination(self),
            ),
            Expr::Bracketed { item, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_expr_ty(item, expr_ty_expectation.clone())
                    .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            )),
            Expr::Unit { .. } => Ok((
                ExprDisambiguation::Trivial,
                Ok(self.term_menu.unit_ty_ontology().into()),
            )),
            Expr::NewTuple { ref items, .. } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                items: ref indices,
                ..
            } => self.calc_index_or_compose_with_list_expr_ty(expr_idx, owner, indices),
            Expr::List { ref items, .. } => {
                Ok(match expr_ty_expectation.disambiguate_ty_path(self) {
                    TypePathDisambiguation::OntologyConstructor => {
                        // ad hoc, assume universe is 1
                        match items.len() {
                            0 => (
                                ListExprDisambiguation::ListFunctor.into(),
                                Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                            ),
                            1 => (
                                ListExprDisambiguation::ArrayFunctor.into(),
                                Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                            ),
                            _ => {
                                print_debug_expr!(self, expr_idx);
                                todo!()
                            }
                        }
                    }
                    TypePathDisambiguation::InstanceConstructor => {
                        let element_ty: FluffyTerm = match expr_ty_expectation
                            .destination_term_data(self.db(), self.fluffy_term_region.terms())
                        {
                            Some(ty_pattern) => match ty_pattern {
                                FluffyTermData::Literal(_) => todo!(),
                                FluffyTermData::TypeOntology {
                                    ty_path: path,
                                    refined_ty_path: refined_path,
                                    arguments,
                                    ..
                                } => match refined_path {
                                    Left(PreludeTypePath::List) => {
                                        assert_eq!(arguments.len(), 1);
                                        arguments[0]
                                    }
                                    Left(PreludeTypePath::Array) => todo!(),
                                    _ => todo!(),
                                },
                                FluffyTermData::Curry {
                                    curry_kind,
                                    variance,
                                    parameter_variable,
                                    parameter_ty,
                                    return_ty,
                                    ty_ethereal_term,
                                } => todo!(),
                                FluffyTermData::Hole(_, _) => todo!(),
                                FluffyTermData::Category(_) => todo!(),
                                FluffyTermData::Ritchie {
                                    ritchie_kind,
                                    parameter_contracted_tys,
                                    return_ty,
                                    ..
                                } => todo!(),
                                FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
                                FluffyTermData::HoleAtPlace {
                                    place,
                                    hole_kind,
                                    hole,
                                } => todo!(),
                                FluffyTermData::Symbol { .. } => todo!(),
                                FluffyTermData::SymbolAtPlace { .. } => todo!(),
                                FluffyTermData::Variable { ty } => todo!(),
                            },
                            None => self.new_hole(expr_idx, HoleKind::ImplicitType).into(),
                        };
                        for item in items {
                            self.infer_new_expr_ty_discarded(
                                item.expr_idx(),
                                ExpectImplicitlyConvertible::new_move(element_ty),
                            );
                        }
                        (
                            ListExprDisambiguation::NewList.into(),
                            FluffyTerm::new_application(
                                self,
                                expr_idx,
                                self.term_menu.list_ty_ontology(),
                                element_ty,
                            )
                            .map_err(|_| todo!()),
                        )
                    }
                })
            }
            Expr::BoxColonList { ref items, .. } => match items.len() {
                0 => Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                )),
                _ => todo!(),
            },
            Expr::Block { stmts } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_block(stmts, expr_ty_expectation.clone())
                    .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            )),
            Expr::EmptyHtmlTag { .. } => Ok((
                ExprDisambiguation::Trivial,
                Ok(self.term_menu.html_ty_ontology().into()),
            )),
            Expr::Ritchie {
                ref parameter_ty_items,
                return_ty_expr,
                ..
            } => {
                for parameter_ty in parameter_ty_items {
                    self.infer_new_expr_ty_discarded(
                        parameter_ty.expr_idx(),
                        self.expect_ty0_subtype(),
                    );
                }
                return_ty_expr.map(|return_ty_expr| {
                    self.infer_new_expr_ty_discarded(return_ty_expr, self.expect_ty0_subtype())
                });
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }

    fn calc_principal_entity_path_expr_ty(
        &mut self,
        path: Option<PrincipalEntityPath>,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let disambiguation = expr_ty_expectation.disambiguate_ty_path(self);
        Ok((
            disambiguation.into(),
            Ok(path
                .ok_or(DerivedExprTypeError::EntityPathError)?
                .ty(self.db, disambiguation)?
                .into()),
        ))
    }

    fn calc_explicit_application(
        &mut self,
        expr_idx: ExprIdx,
        function_expr_idx: ExprIdx,
        argument_expr_idx: ExprIdx,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        Ok((
            ExprDisambiguation::Trivial,
            self.calc_function_application_expr_ty(
                expr_idx,
                function_expr_idx,
                argument_expr_idx,
                final_destination,
            ),
        ))
    }
}
