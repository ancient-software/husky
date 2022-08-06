mod impl_ty_linkage_entries;

use husky_entity_route::entity_route_menu;
use husky_entity_route::{make_subroute, make_type_as_trait_member_route};
use husky_entity_semantics::{DefinitionRepr, FieldDefnVariant, MethodDefnKind};
use husky_word::RootIdentifier;
use infer_decl::{
    CallFormDecl, OutputDecl, ParameterDecl, TraitMemberImplDecl, TyDecl, VariadicTemplate,
};

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_init_content(&mut self) {
        self.write(
            r#"use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &["#,
        );
        let main_module = self.db.module(self.target_entrance).unwrap();
        let entity_link_dependees = self.db.entity_link_dependees(main_module);
        for entity_route in entity_link_dependees.iter() {
            if !entity_route.contains_any() {
                let entity_defn = self.db.entity_defn(*entity_route).unwrap();
                self.gen_linkage_entry(*entity_route, &entity_defn);
            }
        }
        self.write(
            r#"
];
"#,
        );
    }

    fn gen_linkage_entry(&mut self, entity_route: EntityRoutePtr, entity_defn: &EntityDefn) {
        if self.db.is_defn_static(entity_route)
            && !self.db.contains_spatial_parameters(entity_route)
        {
            return;
        }
        match entity_defn.variant {
            EntityDefnVariant::Module { .. } => (),
            EntityDefnVariant::Feature { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { ref expr } => (),
                DefinitionRepr::LazyBlock { ref stmts, ty } => (),
                DefinitionRepr::FuncBlock {
                    route, output_ty, ..
                } => self.gen_eager_feature_linkage_entry(route, output_ty.route),
                DefinitionRepr::ProcBlock {
                    file,
                    range,
                    ref stmts,
                    ty,
                } => todo!(),
            },
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
            EntityDefnVariant::Method { .. } => {
                self.gen_method_linkage_entry(entity_route);
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_specific_routine_linkage(
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Proc { .. } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_specific_routine_linkage(
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Ty {
                ty_kind,
                ref members,
                ref opt_type_call,
                ..
            } => match ty_kind {
                TyKind::Record => (),
                _ => self.gen_ty_linkages(ty_kind, opt_type_call, entity_route, members),
            },
            EntityDefnVariant::Trait {
                ref spatial_parameters,
                ref members,
            } => todo!(),
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty: ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => panic!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {}
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Input { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        }
    }

    fn gen_method_linkage_entry(&mut self, entity_route: EntityRoutePtr) {
        self.write(
            r#"
    ("#,
        );
        match entity_route.variant {
            EntityRouteVariant::Child { parent, ident } => {
                self.write(&format!(
                    r#"
        __StaticLinkageKey::Routine {{ route: "{entity_route}" }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                let this_liason = call_form_decl.this_liason();
                match this_liason {
                    ParameterLiason::MemberAccess => {
                        self.write(&format!(
                            r#"
        method_elem_linkage!("#
                        ));
                        self.gen_entity_route(entity_route.parent(), EntityRouteRole::Decl);
                        let method_name = entity_route.ident().as_str();
                        let mangled_ty_vtable = self.db.mangled_ty_vtable(parent);
                        let mangled_output_ty_vtable =
                            self.db.mangled_ty_vtable(call_form_decl.output.ty);
                        self.write(&format!(
                            ", __registration__::{mangled_ty_vtable}, __registration__::{mangled_output_ty_vtable}, {method_name})"
                        ))
                    }
                    _ => self.gen_specific_routine_linkage(
                        Some((this_liason, entity_route.parent())),
                        |this| {
                            this.write(&format!("__this.{}", entity_route.ident().as_str()));
                        },
                        |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                        &call_form_decl,
                    ),
                }
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                if trai.variant == entity_route_menu().std_ops_index_trai.variant {
                    let this_ty_decl = self.db.ty_decl(ty).unwrap();
                    let trai_impl = this_ty_decl.trait_impl(trai).unwrap();
                    let elem_ty = match trai_impl.member_impls[0] {
                        TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
                        _ => panic!(),
                    };
                    self.gen_index_linkage(ty, elem_ty)
                } else {
                    todo!()
                }
            }
            EntityRouteVariant::Root { ident } => todo!(),
            EntityRouteVariant::Package { main, ident } => todo!(),
            EntityRouteVariant::CrateInputValue { main } => todo!(),
            EntityRouteVariant::Any {
                ident, entity_kind, ..
            } => todo!(),
            EntityRouteVariant::ThisType => todo!(),
            EntityRouteVariant::CrateOutputType { main } => todo!(),
        }
        self.write(
            r#"
    ),"#,
        );
    }

    fn gen_specific_routine_linkage(
        &mut self,
        opt_this: Option<(ParameterLiason, EntityRoutePtr)>,
        gen_caller: impl FnOnce(&mut Self),
        gen_call_route: impl FnOnce(&mut Self),
        decl: &CallFormDecl,
    ) {
        let argidx_base = opt_this.map(|_| 1).unwrap_or(0);
        self.write(&format!(
            r#"
        __Linkage::Transfer(__LinkageFp {{
            dev_src: static_dev_src!(),
            wrapper: {{
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {{"#
        ));
        if let Some((this_liason, this_ty)) = opt_this {
            let mangled_this_ty_vtable = self.db.mangled_ty_vtable(this_ty);
            match this_liason {
                ParameterLiason::Pure => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&");
                        self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_temp_ref(&__registration__::{mangled_this_ty_vtable});"))
                    }
                }
                ParameterLiason::Move => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => panic!(),
                ParameterLiason::EvalRef => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&'eval ");
                        self.gen_entity_route(this_ty.deref_route(), EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_eval_ref(&__registration__::{mangled_this_ty_vtable});"))
                    }
                }
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    self.write("&mut ");
                    self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                    self.write(&format!(
                        " = unsafe {{ __arb_ref(&__arguments[0]) }}.downcast_temp_mut(&__registration__::{mangled_this_ty_vtable});"
                    ))
                }
            }
        }
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            self.gen_parameter_downcast(i + argidx_base, parameter)
        }
        msg_once!("keyword parameter overrides");
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            let parameter_name = parameter.ident;
            let parameter_ty = parameter.ty;
            self.write(&format!(
                r#"
                    let {parameter_name}: "#
            ));
            self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
            self.write(&format!(" = todo!();"))
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { variadic_ty } => {
                let variadic_start = decl.variadic_start();
                let move_or_copy = match self.db.is_copyable(variadic_ty).unwrap() {
                    true => "copy",
                    false => "move",
                };
                let variadic_ty_vtable = self.db.mangled_ty_vtable(variadic_ty);
                self.write(&format!(
                    r#"
                    let __variadics = 
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_{move_or_copy}(&__registration__::{variadic_ty_vtable}))
                            .collect();"#,
                ));
            }
        }
        self.write(&format!(
            r#"
                    "#
        ));
        let is_output_ty_primitive = decl.output.ty.is_primitive();
        if !is_output_ty_primitive {
            self.write("__Register::new_box(");
        }
        gen_caller(self);
        self.write("(");
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&parameter.ident)
        }
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            if i + decl.primary_parameters.len() > 0 {
                self.write(", ");
                if i == 0 {
                    self.write("/* keyword arguments */ ");
                }
            }
            self.write(&parameter.ident)
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { .. } => {
                if decl.primary_parameters.len() > 0 || decl.keyword_parameters.len() > 0 {
                    self.write(", ")
                }
                self.write("__variadics")
            }
        }
        let mangled_output_ty_vtable = self.db.mangled_ty_vtable(decl.output.ty);
        if is_output_ty_primitive {
            self.write(&format!(
                r#").to_register()
                }}
                __wrapper
            }},
            opt_fp: Some("#
            ));
        } else {
            self.write(&format!(
                r#"), &__registration__::{mangled_output_ty_vtable})
                }}
                __wrapper
            }},
            opt_fp: Some("#
            ));
        }
        gen_call_route(self);
        self.write(
            r#" as *const ()),
        }),"#,
        );
    }

    fn gen_eager_feature_linkage_entry(
        &mut self,
        route: EntityRoutePtr,
        output_ty: EntityRoutePtr,
    ) {
        self.write(&format!(
            r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}"
        }},
        {}feature_linkage!("#,
            match output_ty.is_option() {
                true => "opt_",
                false => "",
            }
        ));
        self.gen_entity_route(route, EntityRouteRole::Caller);
        self.write(", __registration__::");
        let entity_defn = self.db.entity_defn(route).unwrap();
        self.write(&self.db.mangled_ty_vtable(output_ty));
        self.write(
            r#"),
    ),"#,
        );
    }

    fn gen_index_linkage(&mut self, ty: EntityRoutePtr, elem_ty: EntityRoutePtr) {
        msg_once!("todo: generic indexing");
        self.write(&format!(
            r#"
        __StaticLinkageKey::Index {{
            opd_tys: &["{ty:?}", "i32"],
        }},"#,
        ));
        self.write(&format!(
            r#"
        index_linkage!("#
        ));
        let mangled_ty_vtable = self.db.mangled_ty_vtable(ty);
        let mangled_elem_ty_vtable = self.db.mangled_ty_vtable(elem_ty);
        self.gen_entity_route(ty, EntityRouteRole::Decl);
        let copy_kind: &'static str = if self.db.is_copyable(ty).unwrap() {
            match ty {
                EntityRoutePtr::Root(root_identifer) => match root_identifer {
                    RootIdentifier::Void
                    | RootIdentifier::I32
                    | RootIdentifier::I64
                    | RootIdentifier::F32
                    | RootIdentifier::F64
                    | RootIdentifier::B32
                    | RootIdentifier::B64
                    | RootIdentifier::Bool => "direct",
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(_) => "box",
                EntityRoutePtr::ThisType => todo!(),
            }
        } else {
            "invalid"
        };
        // ad hoc
        let mutability = match ty.variant {
            EntityRouteVariant::Root { ident } => match ident {
                RootIdentifier::Void => todo!(),
                RootIdentifier::I32 => todo!(),
                RootIdentifier::I64 => todo!(),
                RootIdentifier::F32 => todo!(),
                RootIdentifier::F64 => todo!(),
                RootIdentifier::B32 => todo!(),
                RootIdentifier::B64 => todo!(),
                RootIdentifier::Bool => todo!(),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => "mutable",
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::Fp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => "mutable",
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::Option => todo!(),
            },
            _ => {
                let route_menu = entity_route_menu();
                if ty.variant == route_menu.std_slice_cyclic_slice.variant {
                    "immutable"
                } else {
                    todo!()
                }
            }
        };
        self.write(format!(
            r#",
    __registration__::{mangled_ty_vtable},
    __registration__::{mangled_elem_ty_vtable},
    {copy_kind},
    {mutability}
)"#
        ))
    }

    fn gen_parameter_downcast(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        match parameter.liason {
            ParameterLiason::Pure => {
                if parameter_ty.is_ref() {
                    todo!()
                } else {
                    if self.db.is_copyable(parameter_ty).unwrap() {
                        self.gen_parameter_downcast_copy(i, parameter)
                    } else {
                        self.gen_parameter_downcast_temp_ref(i, parameter)
                    }
                }
            }
            ParameterLiason::Move => self.gen_parameter_downcast_move(i, parameter),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::EvalRef => self.gen_parameter_downcast_eval_ref(i, parameter),
            ParameterLiason::TempRef => todo!(),
            ParameterLiason::TempRefMut => todo!(),
        }
    }

    fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        match parameter_ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::I64
                | RootIdentifier::F32
                | RootIdentifier::F64
                | RootIdentifier::B32
                | RootIdentifier::B64 => self.write(&format!(
                    " = __arguments[{i}].downcast_{root_identifier}();"
                )),
                RootIdentifier::Bool => todo!(),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::Fp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                let parameter_ty_decl: Arc<TyDecl> = self.db.ty_decl(parameter_ty).unwrap();
                match parameter_ty_decl.ty_kind {
                    TyKind::Enum => self.write(&format!(
                        " = __arguments[{i}].downcast_temp_ref::<__VirtualEnum>(&__registration__::__VIRTUAL_ENUM_VTABLE).kind_idx.into();"
                    )),
                    TyKind::Record => todo!(),
                    TyKind::Struct => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Slice => todo!(),
                    TyKind::CyclicSlice => todo!(),
                    TyKind::Tuple => todo!(),
                    TyKind::Mor => todo!(),
                    TyKind::Fp => todo!(),
                    TyKind::AssociatedAny => todo!(),
                    TyKind::ThisAny => todo!(),
                    TyKind::SpatialPlaceholderAny => todo!(),
                    TyKind::BoxAny => todo!(),
                    TyKind::HigherKind => todo!(),
                    TyKind::Ref => todo!(),
                    TyKind::Option => todo!(),
                }
            }
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    fn gen_parameter_downcast_move(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        let mangled_parameter_ty_vtable = self.db.mangled_ty_vtable(parameter_ty);
        self.write(&format!(
            " = unsafe {{ __arb_ref(&__arguments[{i}]) }}.downcast_move(&__registration__::{mangled_parameter_ty_vtable});"
        ))
    }

    fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &"#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        let mangled_parameter_ty_vtable = self.db.mangled_ty_vtable(parameter_ty);
        self.write(&format!(
            " = __arguments[{i}].downcast_temp_ref(&__registration__::{mangled_parameter_ty_vtable});"
        ))
    }

    fn gen_parameter_downcast_eval_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &'eval "#
        ));
        let mangled_parameter_ty_vtable = self.db.mangled_ty_vtable(parameter_ty);
        self.gen_entity_route(parameter_ty.deref_route(), EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_eval_ref(&__registration__::{mangled_parameter_ty_vtable});"))
    }
}
