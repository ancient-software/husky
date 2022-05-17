use check_utils::should_eq;
use entity_kind::TyKind;
use file::FilePtr;
use linkage_table::ResolveLinkage;
use pack_semantics::PackageQueryGroup;
use vm::{BoxedValue, EvalValue, Linkage, MemberValue, StackValue, VMRuntimeResult};

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup:
    EntityDefnQueryGroup + PackageQueryGroup + ResolveLinkage
{
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Option<Arc<InstructionSheet>>;
    fn method_opt_instruction_sheet(
        &self,
        member_route: EntityRoutePtr,
    ) -> Option<Arc<InstructionSheet>>;
    fn dataset_config_instruction_sheet(&self, pack_main: FilePtr) -> Arc<InstructionSheet>;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let entity_defn = db.entity_defn(route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern { .. } => todo!(),
        EntityDefnVariant::Func {
            ref input_placeholders,
            ref stmts,
            ..
        } => Some(InstructionSheetBuilder::new_func(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident.ident),
            stmts,
            false,
        )),
        EntityDefnVariant::Proc {
            ref input_placeholders,
            ref stmts,
            ..
        } => Some(InstructionSheetBuilder::new_impr(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident.ident),
            stmts,
            false,
        )),
        EntityDefnVariant::Type { .. } => todo!(),
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        EntityDefnVariant::Method { .. } => todo!(),
        EntityDefnVariant::Trait { .. } => todo!(),
    }
}

fn method_opt_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let ty = member_route.parent();
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern {} => todo!(),
        EntityDefnVariant::Func { .. } => todo!(),
        EntityDefnVariant::Proc { .. } => todo!(),
        EntityDefnVariant::Type {
            ty_members: ref type_members,
            ref variants,
            kind,
            ref trait_impls,
            ref members,
            ..
        } => {
            let method_defn = db.member_defn(member_route);
            match method_defn.variant {
                EntityDefnVariant::Method {
                    ref method_variant,
                    ref input_placeholders,
                    ..
                } => {
                    let inputs = input_placeholders
                        .iter()
                        .map(|input_placeholder| input_placeholder.ident.ident);
                    let source = match method_variant {
                        MethodDefnVariant::TypeMethod { ty, method_source } => method_source,
                        MethodDefnVariant::TraitMethod {
                            trai,
                            opt_default_source,
                        } => todo!(),
                        MethodDefnVariant::TraitMethodImpl { trai, opt_source } => todo!(),
                    };
                    match source {
                        MethodSource::Func { stmts } => {
                            Some(InstructionSheetBuilder::new_func(db, inputs, stmts, true))
                        }
                        MethodSource::Proc { stmts } => todo!(),
                        MethodSource::Pattern { stmts } => todo!(),
                        MethodSource::Static(_) => None,
                    }
                }
                _ => panic!(),
            }
        }
        EntityDefnVariant::Builtin => todo!(),
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        EntityDefnVariant::Method { .. } => todo!(),
        EntityDefnVariant::Trait { .. } => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    pack_main: FilePtr,
) -> Arc<InstructionSheet> {
    let pack = db.package(pack_main).unwrap();
    InstructionSheetBuilder::new_func(db, vec![].into_iter(), &pack.config.dataset.stmts, false)
}
