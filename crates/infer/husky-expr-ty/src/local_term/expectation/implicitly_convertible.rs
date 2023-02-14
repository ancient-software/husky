use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertibleResolvedOk {
    implicit_conversion: ImplicitConversion,
    expectee: LocalTerm,
    destination: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectImplicitlyConvertibleResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::ImplicitlyConvertible(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl From<ExpectImplicitlyConvertible> for LocalTermExpectation {
    fn from(value: ExpectImplicitlyConvertible) -> Self {
        LocalTermExpectation::ImplicitlyConversion {
            destination: value.destination,
        }
    }
}

impl From<ExpectImplicitlyConvertibleResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectImplicitlyConvertibleResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::ImplicitlyConvertible(value)
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type ResolvedOk = ExpectImplicitlyConvertibleResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_implicit_conversion_expectation(
        &self,
        expectee: LocalTerm,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        let table = self.local_term_table();
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => match destination {
                LocalTerm::Resolved(destination) => self.res_to_res(resolved_expectee, destination),
                LocalTerm::Unresolved(dst) => match table[dst].unresolved_term() {
                    UnresolvedTerm::ImplicitSymbol(_) => match level {
                        LocalTermResolveLevel::Weak => None,
                        LocalTermResolveLevel::Strong => Some(LocalTermExpectationResolvedOkM {
                            actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                                implicit_symbol: dst,
                                substitution: resolved_expectee.into(),
                            }],
                            result: Ok(LocalTermExpectationResolvedOk::ImplicitlyConvertible(
                                ExpectImplicitlyConvertibleResolvedOk {
                                    implicit_conversion: ImplicitConversion::None,
                                    expectee,
                                    destination: expectee,
                                },
                            )),
                        }),
                    },
                    UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                },
            },
            LocalTerm::Unresolved(expectee) => self.unres_to(expectee, destination, level),
        }
    }

    fn res_to_res(
        &self,
        expectee: ReducedTerm,
        destination: ReducedTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        if expectee == destination {
            return Some(LocalTermExpectationResolvedOkM {
                result: Ok(ExpectImplicitlyConvertibleResolvedOk {
                    implicit_conversion: ImplicitConversion::None,
                    expectee: expectee.into(),
                    destination: destination.into(),
                }
                .into()),
                actions: vec![],
            });
        }
        // ad hoc
        Some(LocalTermExpectationResolvedOkM {
            result: Err(OriginalLocalTermExpectationError::Todo.into()),
            actions: vec![],
        })
    }

    fn unres_to(
        &self,
        expectee: UnresolvedTermIdx,
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        let table = self.local_term_table();
        match table[expectee].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationResolvedOkM {
                    result: Ok(LocalTermExpectationResolvedOk::ImplicitlyConvertible(
                        ExpectImplicitlyConvertibleResolvedOk {
                            implicit_conversion: ImplicitConversion::None,
                            expectee: destination.into(),
                            destination: destination.into(),
                        },
                    )),
                    actions: vec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: expectee,
                        substitution: destination,
                    }],
                }),
            },
            UnresolvedTerm::TypeApplication { ty, arguments } => {
                self.unres_ty_app_to(*ty, arguments, destination)
            }
        }
    }

    /// expectation rule effect for implicit conversion from unresolved type application to unresolved expectee
    fn unres_ty_app_to(
        &self,
        ty: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match ty {
            ty if ty == self.entity_path_menu().ref_ty_path() => {
                todo!()
            }
            ty if ty == self.entity_path_menu().ref_mut_ty_path() => {
                todo!()
            }
            ty => self.generic_unres_ty_app_to(ty, arguments, destination),
        }
    }

    fn generic_unres_ty_app_to(
        &self,
        ty_path: TypePath,
        arguments: &[LocalTerm],
        destination: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match destination {
            LocalTerm::Resolved(destination) => {
                let destination_expansion = self.db().application_expansion(destination);
                match destination_expansion.f() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::Entity(destination_ty_path) => {
                        match destination_ty_path.ty_path() {
                            Some(destination_ty_path) if destination_ty_path==ty_path =>(),
                            Some(destination_ty_path) /* if destination_ty_path!=ty_path */ => {
                                return Some(LocalTermExpectationResolvedOkM {
                                    result: Err(OriginalLocalTermExpectationError::Todo.into()),
                                    actions: vec![],
                                })
                            },
                            None => todo!(),
                        };
                        let destination_arguments = &destination_expansion.arguments(self.db());
                        if arguments.len() != destination_arguments.len() {
                            return Some(LocalTermExpectationResolvedOkM {
                                result: Err(OriginalLocalTermExpectationError::Todo.into()),
                                actions: vec![],
                            });
                        };
                        p!(ty_path.debug(self.db()));
                        // ad hoc
                        return Some(LocalTermExpectationResolvedOkM {
                            result: Err(OriginalLocalTermExpectationError::Todo.into()),
                            actions: vec![],
                        });
                        let ty_path_variances = todo!();
                        let actions = std::iter::zip(
                            arguments.iter().copied(),
                            destination_arguments.iter().copied(),
                        )
                        .map(|(argument, destination)| todo!())
                        .collect();
                        Some(LocalTermExpectationResolvedOkM {
                            result: Err(OriginalLocalTermExpectationError::Todo.into()),
                            actions,
                        })
                    }
                    Term::Category(_) => todo!(),
                    Term::Universe(_) => todo!(),
                    Term::Curry(_) => todo!(),
                    Term::Ritchie(_) => todo!(),
                    Term::Abstraction(_) => todo!(),
                    Term::Application(_) => todo!(),
                    Term::Subentity(_) => todo!(),
                    Term::AsTraitSubentity(_) => todo!(),
                    Term::TraitConstraint(_) => todo!(),
                }
            }
            LocalTerm::Unresolved(_) => todo!(),
        }
    }
}
