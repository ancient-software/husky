use crate::*;
use husky_ast::{MatchLiason, ReturnKind};
use husky_entity_route::{EntityRoutePtr, EntityRouteVariant};
use husky_text::TextRange;
use husky_word::RootIdentifier;
use infer_decl::DeclQueryGroup;
use infer_error::throw;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    Move,
    Pass,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl EagerContract {
    pub(crate) fn argument_eager_contract(
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
        output_liason: OutputLiason,
        range: TextRange,
    ) -> EagerContract {
        match parameter_ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => EagerContract::EvalRef,
            _ => match output_liason {
                OutputLiason::Transfer => match parameter_liason {
                    ParameterLiason::Pure => EagerContract::Pure,
                    ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                    ParameterLiason::TempRefMut => EagerContract::TempRefMut,
                    ParameterLiason::MemberAccess => panic!(),
                    ParameterLiason::EvalRef => EagerContract::EvalRef,
                    ParameterLiason::TempRef => todo!(),
                },
                OutputLiason::MemberAccess { .. } => EagerContract::Pure,
            },
        }
    }

    pub(crate) fn method_call_this_eager_contract(
        parameter_liason: ParameterLiason,
        output_liason: OutputLiason,
        output_contract: EagerContract,
    ) -> EagerContract {
        match output_liason {
            OutputLiason::Transfer => match parameter_liason {
                ParameterLiason::Pure => EagerContract::Pure,
                ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                ParameterLiason::TempRefMut => EagerContract::TempRefMut,
                ParameterLiason::MemberAccess => panic!(),
                ParameterLiason::EvalRef => EagerContract::EvalRef,
                ParameterLiason::TempRef => todo!(),
            },
            OutputLiason::MemberAccess { .. } => output_contract,
        }
    }

    pub fn field_access_this_eager_contract(
        field_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_copyable: bool,
        range: TextRange,
    ) -> InferResult<EagerContract> {
        // infer this contract
        if is_member_copyable {
            Ok(match member_contract {
                EagerContract::Pure => EagerContract::Pure,
                EagerContract::Move => panic!(),
                EagerContract::EvalRef => EagerContract::EvalRef,
                EagerContract::TempRef | EagerContract::TempRefMut => match field_liason {
                    MemberLiason::Immutable => {
                        throw!(
                            format!("can't turn a copyable immutable member into temp ref (mut)"),
                            range
                        )
                    }
                    MemberLiason::Mutable => EagerContract::TempRefMut,
                    MemberLiason::Derived => {
                        throw!(
                            format!("can't turn a copyable derived member into temp ref (mut)"),
                            range
                        )
                    }
                },
                EagerContract::Pass => panic!(),
            })
        } else {
            match field_liason {
                MemberLiason::Immutable => match member_contract {
                    EagerContract::TempRefMut => throw!(
                        format!("can't bind mutable reference to an immutable field"),
                        range
                    ),
                    _ => Ok(member_contract),
                },
                MemberLiason::Mutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),
                    EagerContract::Move => Ok(EagerContract::Move),
                    EagerContract::TempRefMut => Ok(EagerContract::TempRefMut),
                    EagerContract::EvalRef => Ok(EagerContract::EvalRef),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => todo!(),
                },
                MemberLiason::Derived => panic!(),
            }
        }
    }

    pub fn from_match(match_liason: MatchLiason) -> Self {
        match match_liason {
            MatchLiason::Pure => EagerContract::Pure,
        }
    }

    pub fn init_contract(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<Self> {
        Ok(if db.is_copyable(ty)? {
            EagerContract::Pure
        } else {
            EagerContract::Pass
        })
    }

    pub fn ret_contract(
        db: &dyn DeclQueryGroup,
        output_ty: EntityRoutePtr,
        return_ty: EntityRoutePtr,
        return_kind: ReturnKind,
    ) -> InferResult<Self> {
        match return_kind {
            ReturnKind::Normal => Ok(if output_ty.variant == return_ty.variant {
                if db.is_copyable(output_ty)? {
                    EagerContract::Pure
                } else {
                    EagerContract::Move
                }
            } else if output_ty.variant
                == (EntityRouteVariant::Root {
                    ident: RootIdentifier::Option,
                })
            {
                if output_ty.spatial_arguments[0].take_entity_route() == return_ty {
                    if db.is_copyable(return_ty)? {
                        EagerContract::Pure
                    } else {
                        EagerContract::Move
                    }
                } else {
                    todo!()
                }
            } else {
                todo!()
            }),
            ReturnKind::Feature | ReturnKind::LazyField => {
                if output_ty.is_ref() {
                    todo!("warn: output ty should be dereferenced")
                } else {
                    if output_ty == return_ty {
                        if db.is_copyable(return_ty)? {
                            Ok(EagerContract::Pure)
                        } else {
                            Ok(EagerContract::Pass)
                        }
                    } else {
                        todo!()
                    }
                }
            }
        }
    }
}
