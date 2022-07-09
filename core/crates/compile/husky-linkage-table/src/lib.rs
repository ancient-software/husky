mod form;
mod key;
mod table;

pub use form::*;
pub use key::*;
pub use table::*;

use check_utils::*;
use colored::Colorize;
use husky_entity_route::{
    entity_route_menu, EntityRoute, EntityRouteKind, EntityRoutePtr, SpatialArgument,
};
use husky_entity_semantics::{CallFormSource, EntityDefnQueryGroup, EntityDefnVariant};
use husky_entity_syntax::EntityLocus;
use husky_file::FilePtr;
use map_collect::MapCollect;
use print_utils::p;
use static_defn::{EntityStaticDefnVariant, FunctionStaticDefnVariant};
use std::collections::HashMap;
use sync_utils::ASafeRwLock;
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;
use vm::{Binding, EntityUid, __Linkage};
use vm::{__EvalResult, __EvalValue, __OwnedValue, __SpecificRoutineLinkage, __TempValue};
use word::{CustomIdentifier, RootIdentifier};

pub trait ResolveLinkage: EntityDefnQueryGroup + Upcast<dyn EntityDefnQueryGroup> {
    fn husky_linkage_table(&self) -> &LinkageTable;

    fn element_access_linkage(&self, opd_tys: Vec<EntityRoutePtr>) -> __Linkage {
        if let Some(__Linkage) = self
            .husky_linkage_table()
            .element_access(self.upcast(), opd_tys.map(|ty| self.entity_uid(*ty)))
        {
            return __Linkage;
        }
        let this_ty_defn = self.entity_defn(opd_tys[0]).unwrap();
        let std_ops_index_trai = self.make_route(
            entity_route_menu().std_ops_index_trai,
            thin_vec![SpatialArgument::EntityRoute(opd_tys[1])],
        );
        let index_trai_impl = this_ty_defn.trait_impl(std_ops_index_trai).unwrap();
        match index_trai_impl.member_impls[1].variant {
            EntityDefnVariant::Method { ref opt_source, .. } => {
                if let Some(source) = opt_source {
                    match source {
                        CallFormSource::Func { stmts } => todo!(),
                        CallFormSource::Proc { stmts } => todo!(),
                        CallFormSource::Lazy { stmts } => todo!(),
                        CallFormSource::Static(__Linkage) => *__Linkage,
                    }
                } else {
                    todo!()
                }
            }
            _ => {
                // p!(method_variant, this_ty_defn.file, this_ty_defn.range);
                panic!()
            }
        }
    }

    fn struct_field_access_linkage(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
        field_binding: Binding,
    ) -> Option<__SpecificRoutineLinkage> {
        if let Some(__Linkage) = self
            .husky_linkage_table()
            .struct_field_access_linkage_source(
                self.upcast(),
                self.entity_uid(this_ty),
                field_ident,
                field_binding,
            )
        {
            return Some(__Linkage.bind(field_binding));
        }
        let this_ty_defn = self.entity_defn(this_ty).unwrap();
        let ty_field_defn = this_ty_defn.field(field_ident);
        match ty_field_defn.variant {
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => opt_linkage.map(|__Linkage| __Linkage.bind(field_binding)),
            _ => panic!(""),
        }
    }

    fn method_linkage(&self, method_route: EntityRoutePtr) -> Option<__Linkage> {
        if let Some(__Linkage) = self
            .husky_linkage_table()
            .routine_linkage(self.upcast(), self.entity_uid(method_route))
        {
            Some(__Linkage)
        } else {
            let method_defn = self.entity_defn(method_route).unwrap();
            match method_defn.variant {
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::Method { ref opt_source, .. } => {
                    if let Some(ref source) = opt_source {
                        match source {
                            CallFormSource::Func { .. }
                            | CallFormSource::Proc { .. }
                            | CallFormSource::Lazy { .. } => None,
                            CallFormSource::Static(linkage_source) => Some(*linkage_source),
                        }
                    } else {
                        todo!()
                        // ,
                        // MethodDefnKind::TraitMethod {
                        //     trai,
                        //     ref opt_default_source,
                        // } => opt_default_source.as_ref().map(|source| match source {
                        //     CallFormSource::Func { stmts } => todo!(),
                        //     CallFormSource::Proc { stmts } => todo!(),
                        //     CallFormSource::Lazy { stmts } => todo!(),
                        //     CallFormSource::Static(linkage_source) => *linkage_source,
                        // }),
                        // MethodDefnKind::TraitMethodImpl { trai, opt_source } => {
                        //     if let Some(source) = opt_source {
                        //         return match source {
                        //             CallFormSource::Func { ref stmts } => todo!(),
                        //             CallFormSource::Proc { ref stmts } => todo!(),
                        //             CallFormSource::Lazy { ref stmts } => todo!(),
                        //             CallFormSource::Static(linkage_source) => Some(*linkage_source),
                        //         };
                        //     }
                        //     let trai_defn = self.entity_defn(*trai).unwrap();
                        //     match trai_defn.variant {
                        //         EntityDefnVariant::Trait {
                        //             ref generic_parameters,
                        //             ref members,
                        //         } => {
                        //             let member = members
                        //                 .iter()
                        //                 .find(|member| member.ident == method_defn.ident)
                        //                 .unwrap();
                        //             match member.variant {
                        //                 EntityDefnVariant::Method {
                        //                     method_variant:
                        //                         MethodDefnKind::TraitMethod {
                        //                             trai,
                        //                             ref opt_default_source,
                        //                         },
                        //                     ..
                        //                 } => match opt_default_source.as_ref().unwrap() {
                        //                     CallFormSource::Func { ref stmts } => todo!(),
                        //                     CallFormSource::Proc { ref stmts } => todo!(),
                        //                     CallFormSource::Lazy { ref stmts } => todo!(),
                        //                     CallFormSource::Static(linkage_source) => {
                        //                         Some(*linkage_source)
                        //                     }
                        //                 },
                        //                 _ => panic!(),
                        //             }
                        //         }
                        //         _ => panic!(),
                        //     }
                        // }
                        // },
                    }
                }
                _ => todo!(),
            }
        }
    }

    fn routine_linkage(&self, routine: EntityRoutePtr) -> Option<__Linkage> {
        match self.entity_locus(routine).unwrap() {
            EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
                EntityStaticDefnVariant::Function { linkage, .. } => Some(linkage),
                _ => todo!(),
            },
            EntityLocus::WithinBuiltinModule => todo!(),
            EntityLocus::WithinModule { .. } => self
                .husky_linkage_table()
                .routine_linkage(self.upcast(), self.entity_uid(routine)),
            EntityLocus::Module { file } => todo!(),
            EntityLocus::Input { main } => todo!(),
            EntityLocus::StaticTypeMember => todo!(),
            EntityLocus::StaticTypeAsTraitMember => todo!(),
        }
    }

    fn type_call_linkage(&self, ty: EntityRoutePtr) -> Option<__Linkage> {
        if let Some(linkage) = self
            .husky_linkage_table()
            .type_call_linkage(self.upcast(), self.entity_uid(ty))
        {
            return Some(linkage);
        }
        let type_defn = self.entity_defn(ty).unwrap();
        match type_defn.variant {
            EntityDefnVariant::Ty {
                ref opt_type_call, ..
            } => opt_type_call
                .as_ref()
                .map(|type_call| type_call.opt_linkage)
                .flatten(),
            _ => panic!(),
        }
    }
}
