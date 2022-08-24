mod field;
// mod method;

pub use field::*;
// pub use method::*;

use fold::LocalStack;
use husky_atom::{context::Symbol, AtomContext};
use husky_print_utils::{epin, p};
use map_collect::MapCollect;
use vec_like::VecMapEntry;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MemberDecl {
    AssociatedType,
    AssociatedCall,
    TypeField(Arc<FieldDecl>),
    TypeMethod(Arc<CallFormDecl>),
    TypeAssociatedCall(Arc<CallFormDecl>),
    TraitMethodImpl {
        trait_route: EntityRoutePtr,
        method: Arc<CallFormDecl>,
    },
    TraitAssociatedTypeImpl {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    TraitAssociatedConstSizeImpl {
        ident: CustomIdentifier,
        value: usize,
    },
}

impl MemberDecl {
    pub fn ident(&self) -> CustomIdentifier {
        match self {
            MemberDecl::AssociatedType => todo!(),
            MemberDecl::AssociatedCall => todo!(),
            MemberDecl::TypeField(field) => field.ident,
            MemberDecl::TypeMethod(method) => method.ident(),
            MemberDecl::TraitMethodImpl { method, .. } => method.ident(),
            MemberDecl::TraitAssociatedTypeImpl { ident, .. } => *ident,
            MemberDecl::TraitAssociatedConstSizeImpl { ident, .. } => *ident,
            MemberDecl::TypeAssociatedCall(call) => call.ident(),
        }
    }

    pub fn opt_route(&self) -> EntityRoutePtr {
        match self {
            MemberDecl::AssociatedType => todo!(),
            MemberDecl::AssociatedCall => todo!(),
            MemberDecl::TypeField(_) => todo!(),
            MemberDecl::TypeMethod(ty_method) => ty_method.opt_route.unwrap(),
            MemberDecl::TypeAssociatedCall(_) => todo!(),
            MemberDecl::TraitMethodImpl {
                trait_route,
                method,
            } => method.opt_route.unwrap(),
            MemberDecl::TraitAssociatedTypeImpl { ident, ty } => *ty,
            MemberDecl::TraitAssociatedConstSizeImpl { ident, value } => todo!(),
        }
    }

    pub(crate) fn from_trait_member_impl(
        trait_route: EntityRoutePtr,
        trait_member_impl: &TraitMemberImplDecl,
    ) -> Self {
        match trait_member_impl {
            TraitMemberImplDecl::Method(method) => MemberDecl::TraitMethodImpl {
                trait_route,
                method: method.clone(),
            },
            TraitMemberImplDecl::AssociatedType { ident, ty } => {
                MemberDecl::TraitAssociatedTypeImpl {
                    ident: *ident,
                    ty: *ty,
                }
            }
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemberIdx(pub u8);

impl From<usize> for MemberIdx {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().unwrap())
    }
}

impl From<&TyMemberDecl> for MemberDecl {
    fn from(decl: &TyMemberDecl) -> Self {
        match decl {
            TyMemberDecl::Field(field_decl) => MemberDecl::TypeField(field_decl.clone()),
            TyMemberDecl::Method(call_form_decl) => MemberDecl::TypeMethod(call_form_decl.clone()),
            TyMemberDecl::Call(call_decl) => MemberDecl::TypeAssociatedCall(call_decl.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyMemberDecl {
    Field(Arc<FieldDecl>),
    Method(Arc<CallFormDecl>),
    Call(Arc<CallFormDecl>),
}

impl TyMemberDecl {
    pub(crate) fn instantiate(&self, instantiator: &InstantiationContext) -> Self {
        match self {
            TyMemberDecl::Field(field_decl) => {
                TyMemberDecl::Field(field_decl.instantiate(instantiator))
            }
            TyMemberDecl::Method(call_form_decl) => {
                TyMemberDecl::Method(call_form_decl.instantiate(instantiator))
            }
            TyMemberDecl::Call(_) => todo!(),
        }
    }

    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        symbol_context: &mut dyn AtomContext,
        route: EntityRoutePtr,
        static_defn: &EntityStaticDefn,
    ) -> InferResult<Self> {
        Ok(match static_defn.variant {
            EntityStaticDefnVariant::Method { .. } => TyMemberDecl::Method(
                CallFormDecl::from_static(db, route, symbol_context, static_defn)?,
            ),
            EntityStaticDefnVariant::TyField { .. } => {
                TyMemberDecl::Field(FieldDecl::from_static(symbol_context, static_defn))
            }
            _ => panic!(""),
        })
    }
}

impl VecMapEntry<CustomIdentifier> for TyMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TyMemberDecl::Method(call_form_decl) => call_form_decl.ident(),
            TyMemberDecl::Field(field_decl) => field_decl.ident,
            TyMemberDecl::Call(call_decl) => call_decl.ident(),
        }
    }
}

impl MemberDecl {
    pub(crate) fn collect_all(
        db: &dyn DeclQueryGroup,
        type_members: &[TyMemberDecl],
        trait_impls: &[Arc<TraitImplDecl>],
    ) -> Vec<MemberDecl> {
        let mut members: Vec<MemberDecl> = type_members.map(|decl| decl.into());
        for trait_impl in trait_impls {
            for member in trait_impl.member_impls() {
                members.push(MemberDecl::from_trait_member_impl(
                    trait_impl.trai(),
                    member,
                ))
            }
        }
        members
    }
}

pub(crate) fn member_idx(db: &dyn DeclQueryGroup, member_route: EntityRoutePtr) -> MemberIdx {
    let this_ty = member_route.parent();
    let this_ty_decl = db.ty_decl(this_ty).unwrap();
    this_ty_decl.member_idx(member_route)
}
