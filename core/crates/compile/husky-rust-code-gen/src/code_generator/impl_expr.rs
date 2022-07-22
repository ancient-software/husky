use super::{impl_entity_route::EntityRouteRole, *};
use fold::Indent;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use husky_entity_semantics::FieldDefnVariant;
use husky_infer_qualified_ty::EagerExprQualifier;
use infer_decl::VariadicTemplate;
use vm::*;
use word::RootIdentifier;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_expr(&mut self, indent: Indent, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => self.write(&varname),
            EagerExprVariant::ThisValue { .. } => self.write("self"),
            EagerExprVariant::ThisField { field_ident, .. } => match self.context {
                RustCodeGenContext::Normal => {
                    self.write("self.");
                    self.write(&field_ident.ident);
                }
                RustCodeGenContext::StructDerivedEager => {
                    self.write(&field_ident.ident);
                }
            },
            EagerExprVariant::PrimitiveLiteral(value) => self.gen_copyable_literal(value),
            EagerExprVariant::Bracketed(ref expr) => {
                self.write("(");
                self.gen_expr(indent, expr);
                self.write(")")
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, .. } => {
                    match opr {
                        BinaryOpr::Pure(_) => (),
                        BinaryOpr::Assign(_) => match opds[0].variant {
                            EagerExprVariant::Variable { varname, binding } => (),
                            EagerExprVariant::Opn {
                                ref opn_variant, ..
                            } => match opn_variant {
                                EagerOpnVariant::Index { .. } => (),
                                EagerOpnVariant::Field { .. } => (),
                                _ => self.write("*"),
                            },
                            _ => self.write("*"),
                        },
                    }
                    self.gen_expr(indent, &opds[0]);
                    match opr {
                        BinaryOpr::Pure(PureBinaryOpr::RemEuclid) => {
                            self.write(".rem_euclid(");
                            self.gen_expr(indent, &opds[1]);
                            self.write(")")
                        }
                        BinaryOpr::Assign(Some(PureBinaryOpr::RemEuclid)) => todo!(),
                        _ => {
                            self.write(opr.spaced_code());
                            self.gen_expr(indent, &opds[1]);
                        }
                    }
                }
                EagerOpnVariant::Prefix { opr, .. } => match opr {
                    PrefixOpr::Not => match opds[0].ty() {
                        EntityRoutePtr::Root(RootIdentifier::Bool) => {
                            self.write("!");
                            self.gen_expr(indent, &opds[0]);
                        }
                        _ => {
                            self.write("(0 == ");
                            self.gen_expr(indent, &opds[0]);
                            self.write(")");
                        }
                    },
                    _ => {
                        self.write(&opr.rust_code());
                        self.gen_expr(indent, &opds[0]);
                    }
                },
                EagerOpnVariant::Suffix { opr, .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.gen_suffix_opr(*opr)
                }
                EagerOpnVariant::RoutineCall(routine) => {
                    self.gen_entity_route(routine.route, EntityRouteRole::Caller);
                    self.write("(");
                    self.gen_arguments(indent, opds);
                    self.write(")");
                }
                EagerOpnVariant::TypeCall {
                    ranged_ty,
                    ref ty_decl,
                    ..
                } => {
                    self.gen_type_call_opn(indent, ranged_ty.route, opds, ty_decl);
                }
                EagerOpnVariant::Field { field_ident, .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.write(".");
                    self.write(&field_ident.ident)
                }
                EagerOpnVariant::MethodCall {
                    method_ident,
                    method_route,
                    output_binding,
                    ..
                } => {
                    let call_form_decl = self.db.entity_call_form_decl(*method_route).unwrap();
                    match call_form_decl.output.liason {
                        OutputLiason::Transfer => {
                            self.gen_expr(indent, &opds[0]);
                            self.write(".");
                            self.write(&method_ident.ident);
                            self.write("(");
                            self.gen_arguments(indent, &opds[1..]);
                            self.write(")");
                        }
                        OutputLiason::MemberAccess { .. } => match output_binding {
                            Binding::EvalRef | Binding::TempRef => {
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::Copy => {
                                self.write("*");
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::TempRefMut => {
                                self.gen_expr(indent, &opds[0]);
                                self.write(".");
                                self.write(&method_ident.ident);
                                self.write("_mut");
                                self.write("(");
                                self.gen_arguments(indent, &opds[1..]);
                                self.write(")");
                            }
                            Binding::Move => todo!(),
                        },
                    }
                }
                EagerOpnVariant::Index { .. } => {
                    self.gen_expr(indent, &opds[0]);
                    self.write("[");
                    if opds.len() > 2 {
                        todo!()
                    } else {
                        self.write("(");
                        self.gen_expr(indent, &opds[1]);
                        self.write(")");
                        self.write(" as usize")
                    }
                    self.write("]");
                }
                EagerOpnVariant::NewVecFromList => todo!(),
                EagerOpnVariant::ValueCall => todo!(),
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EnumKindLiteral(value) => {
                self.gen_entity_route(value, EntityRouteRole::Other)
            }
            EagerExprVariant::EntityFeature { route } => {
                self.gen_entity_route(route, EntityRouteRole::Caller);
                self.write("(__ctx)")
            }
            EagerExprVariant::EntityFp { route } => todo!(),
        }
    }

    fn gen_type_call_opn(
        &mut self,
        indent: Indent,
        ty: EntityRoutePtr,
        opds: &Vec<Arc<EagerExpr>>,
        ty_decl: &Arc<infer_decl::TyDecl>,
    ) {
        let type_call = ty_decl.opt_type_call.as_ref().unwrap();
        let needs_wrapping = type_call.keyword_parameters.len() > 0;
        if needs_wrapping {
            self.write("{\n");
            self.indent(indent + 8);
        }
        let ty_defn = self.db.entity_defn(ty).unwrap();
        let ty_members = match ty_defn.variant {
            EntityDefnVariant::Ty {
                ref spatial_parameters,
                ref ty_members,
                ref variants,
                ty_kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                opt_static_visual_ty,
                ref opt_visual_stmts,
            } => ty_members,
            _ => panic!(),
        };
        for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
            self.write("let __keyword_");
            self.write(&parameter.ident);
            match ty_members.data()[type_call.primary_parameters.len() + i].variant {
                EntityDefnVariant::TyField {
                    ty,
                    ref field_variant,
                    liason,
                    opt_linkage,
                } => match field_variant {
                    FieldDefnVariant::StructDefault { default } => todo!(),
                    _ => panic!(),
                },
                _ => panic!(),
            }
            self.write(" = todo!();");
            self.newline_indented(indent + 8);
        }
        self.gen_entity_route(ty, EntityRouteRole::Caller);
        self.write("::");
        self.write("__call__(");
        self.gen_arguments(indent, opds);
        for (i, parameter) in type_call.keyword_parameters.iter().enumerate() {
            if i + type_call.primary_parameters.len() > 0 {
                self.write(", ")
            }
            self.write("__keyword_");
            self.write(&parameter.ident)
        }
        msg_once!("keyword arguments and more on variadics");
        let type_call_decl = &ty_decl.opt_type_call.as_ref().unwrap();
        match type_call_decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { ty } => {
                if type_call_decl.primary_parameters.len() + type_call_decl.keyword_parameters.len()
                    > 0
                {
                    self.write(", ")
                }
                self.write("vec![]")
            }
        }
        self.write(")");
        if needs_wrapping {
            self.newline_indented(indent + 4);
            self.write("}");
        }
    }

    pub(super) fn gen_feature_return(&mut self, indent: Indent, result: &EagerExpr) {
        match result.qualified_ty.qual {
            EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
                self.write(
                    r#"__cache_feature(
        __ctx,
        __feature,
        Ok(("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#").__into_eval_value())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::EvalRef => {
                self.write(
                    r#"__cache_feature(
        __ctx,
        __feature,
        Ok(__EvalRef(&("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#")).into())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::PureRef
            | EagerExprQualifier::TempRef
            | EagerExprQualifier::TempRefMut => panic!(),
        }
    }

    pub(super) fn gen_lazy_field_return(&mut self, indent: Indent, result: &EagerExpr) {
        match result.qualified_ty.qual {
            EagerExprQualifier::Copyable | EagerExprQualifier::Transient => {
                self.write(
                    r#"__cache_lazy_field(
        __ctx,
        self,
        __uid,
        Ok(("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#").__into_eval_value())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::EvalRef => {
                self.write(
                    r#"__cache_lazy_field(
        __ctx,
        self,
        __uid,
        Ok(__EvalRef(&("#,
                );
                self.gen_expr(indent, result);
                self.write(
                    r#")).into())
    ).unwrap()"#,
                );
            }
            EagerExprQualifier::PureRef
            | EagerExprQualifier::TempRef
            | EagerExprQualifier::TempRefMut => panic!(),
        }
    }

    fn gen_arguments(&mut self, indent: Indent, exprs: &[Arc<EagerExpr>]) {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.gen_binding(expr);
            self.gen_expr(indent, expr)
        }
    }

    pub(super) fn gen_binding(&mut self, expr: &EagerExpr) {
        match expr.qualified_ty.qual.binding(expr.contract) {
            Binding::EvalRef => (),
            Binding::TempRef => self.write("&"),
            Binding::TempRefMut => self.write("&mut "),
            Binding::Move => (),
            Binding::Copy => (),
        }
    }

    fn gen_copyable_literal(&mut self, v: CopyableValue) {
        match v {
            CopyableValue::I32(i) => {
                self.result.push_str(&i.to_string());
            }
            CopyableValue::F32(f) => {
                self.result.push_str(&f.to_string());
                self.write("f32")
            }
            CopyableValue::B32(b) => {
                self.result.push_str(&b.to_string());
                self.write("u32")
            }
            CopyableValue::B64(b) => {
                self.result.push_str(&b.to_string());
                self.write("u64")
            }
            CopyableValue::Bool(b) => self.result.push_str(&b.to_string()),
            CopyableValue::Void(_) => self.result.push_str("()"),
            CopyableValue::EnumKind(_) => todo!(),
        }
    }

    fn gen_suffix_opr(&mut self, opr: SuffixOpr) {
        match opr {
            SuffixOpr::Incr => self.write(" += 1"),
            SuffixOpr::Decr => self.write(" -= 1"),
            SuffixOpr::AsTy(ty) => {
                self.write(" as ");
                self.gen_entity_route(ty.route, EntityRouteRole::Other)
            }
        }
    }
}
