use husky_entity_route::EntityRouteVariant;
use husky_lazy_semantics::LazyConditionBranchVariant;
use husky_print_utils::msg_once;
use husky_semantics_error::*;
use vec_like::{VecMap, VecPairMap};

use crate::*;

pub type DependeeMap = VecMap<Ty, (Ty, EntityDefnUid)>;

pub struct DependeeMapBuilder<'a> {
    db: &'a dyn EntityDefnQueryGroup,
    map: VecMap<Ty, (Ty, EntityDefnUid)>,
}

impl<'a> DependeeMapBuilder<'a> {
    fn new(db: &'a dyn EntityDefnQueryGroup) -> DependeeMapBuilder<'a> {
        Self {
            db,
            map: Default::default(),
        }
    }

    fn push(&mut self, entity_route: Ty) {
        let entity_route = entity_route.intrinsic();
        match entity_route.variant {
            EntityRouteVariant::Root { ident, .. } => {
                if ident == RootBuiltinIdentifier::Ref || ident == RootBuiltinIdentifier::Option {
                    panic!()
                } else {
                    if entity_route.spatial_arguments.len() == 0 {
                        return;
                    }
                }
            }
            EntityRouteVariant::TargetInputValue => return,
            EntityRouteVariant::Package { .. } => todo!(),
            EntityRouteVariant::Child { .. } => {
                msg_once!("dependences on entity from external packs should be merged");
                ()
            }
            EntityRouteVariant::Any { .. } => todo!(),
            EntityRouteVariant::ThisType { .. } => todo!(),
            EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
        }
        if !self.map.has(entity_route) {
            self.map
                .insert_new((entity_route, self.db.entity_defn_uid(entity_route)))
                .unwrap();
        }
    }

    fn finish(self) -> DependeeMap {
        self.map
    }
}

pub(crate) fn entity_immediate_dependees(
    db: &dyn EntityDefnQueryGroup,
    entity_route: Ty,
) -> SemanticResultArc<DependeeMap> {
    let defn = db.entity_defn(entity_route)?;
    Ok(Arc::new(defn.immediate_dependees(db)))
}

pub(crate) fn entity_dependees(
    db: &dyn EntityDefnQueryGroup,
    entity_route: Ty,
) -> SemanticResultArc<DependeeMap> {
    let mut dependees = (*db.entity_immediate_dependees(entity_route)?).clone();
    visit_all(db, &mut dependees, 0)?;
    return Ok(Arc::new(dependees));

    fn visit_all(
        db: &dyn EntityDefnQueryGroup,
        map: &mut DependeeMap,
        start: usize,
    ) -> SemanticResult<()> {
        let len0 = map.len();
        for i in start..len0 {
            let (subroute, _) = map.data()[i];
            let submap = db.entity_immediate_dependees(subroute)?;
            map.extend_from_ref(&submap);
        }
        if map.len() > len0 {
            visit_all(db, map, len0)
        } else {
            Ok(())
        }
    }
}

impl EntityDefn {
    fn immediate_dependees(&self, db: &dyn EntityDefnQueryGroup) -> VecPairMap<Ty, EntityDefnUid> {
        let mut builder = DependeeMapBuilder::new(db);
        match self.variant {
            EntityDefnVariant::Module {
                ref module_items, ..
            } => {
                for item in module_items.iter() {
                    builder.push(item.base_route)
                }
            }
            EntityDefnVariant::Feature { ref defn_repr } => {
                extract_defn_repr_dependees(defn_repr, &mut builder);
            }
            EntityDefnVariant::Func {
                ref parameters,
                output,
                ref stmts,
                ..
            } => {
                extract_call_head_dependees(parameters, output, &mut builder);
                extract_func_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Proc {
                ref parameters,
                output,
                ref stmts,
                ..
            } => {
                extract_call_head_dependees(parameters, output, &mut builder);
                extract_proc_stmts_dependees(stmts, &mut builder);
            }
            EntityDefnVariant::Ty {
                ref ty_members,
                ref variants,
                ..
            } => {
                ty_members.iter().for_each(|member| match member.variant {
                    EntityDefnVariant::TyField { field_ty: ty, .. } => builder.push(ty),
                    _ => (),
                });
                variants.iter().for_each(|enum_variant| {
                    extract_enum_variant_dependees(enum_variant, &mut builder)
                });
            }
            EntityDefnVariant::Builtin => (),
            EntityDefnVariant::EnumVariant {
                enum_variant_defn_variant: ref variant,
                ..
            } => match variant {
                EnumVariantDefnVariant::Constant => (),
            },
            EntityDefnVariant::TyField {
                field_ty: ty,
                ref field_variant,
                ..
            } => {
                builder.push(ty);
                match field_variant {
                    FieldDefnVariant::StructOriginal => todo!(),
                    FieldDefnVariant::RecordOriginal => todo!(),
                    FieldDefnVariant::StructDerivedLazy { defn_repr } => {
                        extract_defn_repr_dependees(defn_repr, &mut builder)
                    }
                    FieldDefnVariant::RecordDerived { defn_repr } => {
                        extract_defn_repr_dependees(defn_repr, &mut builder)
                    }
                    FieldDefnVariant::StructDefault { .. } => todo!(),
                    FieldDefnVariant::StructDerivedEager { .. } => todo!(),
                }
            }
            EntityDefnVariant::Method {
                ref parameters,
                output_ty,
                // ref method_variant,
                ref opt_source,
                ..
            } => {
                extract_call_head_dependees(parameters, output_ty, &mut builder);
                // let opt_source = match method_variant {
                //     MethodDefnKind::TypeMethod { ty, method_source } => {
                //         builder.push(*ty);
                //         Some(method_source)
                //     }
                //     MethodDefnKind::TraitMethod {
                //         trai,
                //         opt_default_source,
                //     } => todo!(),
                //     MethodDefnKind::TraitMethodImpl { trai, opt_source } => todo!(),
                // };
                if let Some(source) = opt_source {
                    match source {
                        CallFormSource::Func { stmts } => {
                            extract_func_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Proc { stmts } => {
                            extract_proc_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Lazy { stmts } => {
                            extract_lazy_stmts_dependees(stmts, &mut builder)
                        }
                        CallFormSource::Static(_) => (),
                    }
                }
            }
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
            EntityDefnVariant::Trait { .. } => todo!(),
            EntityDefnVariant::Function { ref source, .. } => match source {
                CallFormSource::Func { stmts } => extract_func_stmts_dependees(stmts, &mut builder),
                CallFormSource::Proc { stmts: _ } => todo!(),
                CallFormSource::Lazy { stmts: _ } => todo!(),
                CallFormSource::Static(_) => (),
            },
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => (),
        };
        return builder.finish();

        fn extract_call_head_dependees(
            inputs: &[Parameter],
            output: RangedEntityRoute,
            builder: &mut DependeeMapBuilder,
        ) {
            for input_placeholder in inputs.iter() {
                builder.push(input_placeholder.ty())
            }
            builder.push(output.route);
        }

        fn extract_defn_repr_dependees(
            defn_repr: &DefinitionRepr,
            builder: &mut DependeeMapBuilder,
        ) {
            match defn_repr {
                DefinitionRepr::LazyExpr { ref expr } => extract_lazy_expr_dependees(expr, builder),
                DefinitionRepr::LazyBlock { stmts, ty: _ } => {
                    extract_lazy_stmts_dependees(stmts, builder)
                }
                DefinitionRepr::FuncBlock { stmts, route, .. } => {
                    builder.push(*route);
                    extract_func_stmts_dependees(stmts, builder)
                }
                DefinitionRepr::ProcBlock { stmts, .. } => {
                    extract_proc_stmts_dependees(stmts, builder)
                }
            }
        }

        fn extract_lazy_stmts_dependees(stmts: &[Arc<LazyStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    LazyStmtVariant::Init { ref value, .. } => {
                        extract_lazy_expr_dependees(value, builder)
                    }
                    LazyStmtVariant::Assert { ref condition } => {
                        extract_lazy_expr_dependees(condition, builder)
                    }
                    LazyStmtVariant::Require { ref condition, .. } => {
                        extract_lazy_expr_dependees(condition, builder)
                    }
                    LazyStmtVariant::Return { ref result } => {
                        extract_lazy_expr_dependees(result, builder)
                    }
                    LazyStmtVariant::ReturnUnveil { ref result, .. } => {
                        extract_lazy_expr_dependees(result, builder)
                    }
                    LazyStmtVariant::ConditionFlow {
                        ref branches,
                        ty: _,
                    } => {
                        for branch in branches {
                            match branch.variant {
                                LazyConditionBranchVariant::If { ref condition } => {
                                    extract_lazy_expr_dependees(condition, builder)
                                }
                                LazyConditionBranchVariant::Elif { ref condition } => {
                                    extract_lazy_expr_dependees(condition, builder)
                                }
                                LazyConditionBranchVariant::Else => (),
                            }
                            extract_lazy_stmts_dependees(&branch.stmts, builder)
                        }
                    }
                    LazyStmtVariant::Match { .. } => todo!(),
                    LazyStmtVariant::ReturnXml { .. } => todo!(),
                }
            }
        }

        fn extract_func_stmts_dependees(stmts: &[Arc<FuncStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    FuncStmtVariant::Init {
                        ref initial_value, ..
                    } => extract_eager_expr_dependees(initial_value, builder),
                    FuncStmtVariant::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, builder)
                    }
                    FuncStmtVariant::Require { ref condition, .. } => {
                        extract_eager_expr_dependees(condition, builder)
                    }
                    FuncStmtVariant::Return { ref result, .. } => {
                        extract_eager_expr_dependees(result, builder)
                    }
                    FuncStmtVariant::ConditionFlow { ref branches } => {
                        for branch in branches {
                            extract_func_stmts_dependees(&branch.stmts, builder)
                        }
                    }
                    FuncStmtVariant::Match {
                        ref match_expr,
                        ref branches,
                    } => {
                        extract_eager_expr_dependees(match_expr, builder);
                        for branch in branches {
                            extract_func_stmts_dependees(&branch.stmts, builder)
                        }
                    }
                }
            }
        }

        fn extract_proc_stmts_dependees(stmts: &[Arc<ProcStmt>], builder: &mut DependeeMapBuilder) {
            for stmt in stmts {
                match stmt.variant {
                    ProcStmtVariant::Init {
                        ref initial_value, ..
                    } => extract_eager_expr_dependees(initial_value, builder),
                    ProcStmtVariant::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, builder)
                    }
                    ProcStmtVariant::Return { ref result, .. } => {
                        extract_eager_expr_dependees(result, builder)
                    }
                    ProcStmtVariant::Execute { ref expr } => {
                        extract_eager_expr_dependees(expr, builder)
                    }
                    ProcStmtVariant::ConditionFlow { ref branches } => {
                        for branch in branches {
                            extract_proc_condition_branch_dependees(branch, builder)
                        }
                    }
                    ProcStmtVariant::Loop {
                        loop_variant: ref loop_kind,
                        ref stmts,
                    } => {
                        match loop_kind {
                            LoopVariant::For {
                                ref initial_boundary,
                                ref final_boundary,
                                ..
                            } => {
                                extract_boundary_dependees(initial_boundary, builder);
                                extract_boundary_dependees(final_boundary, builder);
                            }
                            LoopVariant::ForExt {
                                ref final_boundary, ..
                            } => {
                                extract_boundary_dependees(final_boundary, builder);
                            }
                            LoopVariant::While { condition } => {
                                extract_eager_expr_dependees(condition, builder)
                            }
                            LoopVariant::DoWhile { condition } => {
                                extract_eager_expr_dependees(condition, builder)
                            }
                        }
                        extract_proc_stmts_dependees(stmts, builder)
                    }
                    ProcStmtVariant::Break => (),
                    ProcStmtVariant::Match {
                        ref match_expr,
                        ref branches,
                    } => {
                        extract_eager_expr_dependees(match_expr, builder);
                        for branch in branches {
                            extract_proc_stmts_dependees(&branch.stmts, builder)
                        }
                    }
                }
            }
        }

        fn extract_lazy_expr_dependees(expr: &LazyExpr, builder: &mut DependeeMapBuilder) {
            match expr.variant {
                LazyExprVariant::Variable { .. } | LazyExprVariant::PrimitiveLiteral(_) => (),
                LazyExprVariant::EnumLiteral { .. } => (),
                LazyExprVariant::Bracketed(ref expr) => extract_lazy_expr_dependees(expr, builder),
                LazyExprVariant::Opn { opn_kind, ref opds } => {
                    match opn_kind {
                        LazyOpnKind::Binary { .. }
                        | LazyOpnKind::Prefix(_)
                        | LazyOpnKind::Field { .. }
                        | LazyOpnKind::MethodCall { .. } => (),
                        LazyOpnKind::FunctionModelCall(routine) => builder.push(routine.route),
                        LazyOpnKind::FunctionRoutineCall(routine) => builder.push(routine.route),
                        LazyOpnKind::StructCall(ty) => builder.push(ty.route),
                        LazyOpnKind::RecordCall(ty) => builder.push(ty.route),
                        LazyOpnKind::Index { .. } => (),
                        LazyOpnKind::NewVecFromList => builder.push(expr.intrinsic_ty()),
                    }
                    for opd in opds {
                        extract_lazy_expr_dependees(opd, builder)
                    }
                }
                LazyExprVariant::Lambda(_, _) => todo!(),
                LazyExprVariant::ThisValue { .. } => (),
                LazyExprVariant::EntityFeature {
                    entity_route: route,
                } => builder.push(route),
                LazyExprVariant::ThisField { .. } => todo!(),
                LazyExprVariant::BePattern { .. } => (),
            }
        }

        fn extract_eager_expr_dependees(expr: &EagerExpr, builder: &mut DependeeMapBuilder) {
            match expr.variant {
                EagerExprVariant::Variable { .. } => (),
                EagerExprVariant::PrimitiveLiteral(_) => (),
                EagerExprVariant::Bracketed(ref expr) => {
                    extract_eager_expr_dependees(expr, builder)
                }
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => {
                    match opn_variant {
                        EagerOpnVariant::Binary { .. }
                        | EagerOpnVariant::Prefix { .. }
                        | EagerOpnVariant::Suffix { .. }
                        | EagerOpnVariant::Field { .. }
                        | EagerOpnVariant::MethodCall { .. }
                        | EagerOpnVariant::Index { .. } => (),
                        EagerOpnVariant::RoutineCall(routine) => builder.push(routine.route),
                        EagerOpnVariant::TypeCall { .. } => {
                            todo!()
                            // builder.push(ranged_ty.route)
                        }
                        EagerOpnVariant::NewVecFromList => (),
                        EagerOpnVariant::ValueCall => (),
                    }
                    for opd in opds {
                        extract_eager_expr_dependees(opd, builder)
                    }
                }
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::ThisValue { .. } => builder.push(expr.intrinsic_ty()),
                EagerExprVariant::ThisField { this_ty, .. } => builder.push(this_ty),
                EagerExprVariant::EnumKindLiteral(_) => builder.push(expr.intrinsic_ty()),
                EagerExprVariant::EntityFeature { route } => builder.push(route),
                EagerExprVariant::EntityThickFp { route } => builder.push(route),
            }
        }

        fn extract_boundary_dependees(boundary: &Boundary, builder: &mut DependeeMapBuilder) {
            boundary
                .opt_bound
                .as_ref()
                .map(|bound| extract_eager_expr_dependees(bound, builder));
        }

        fn extract_enum_variant_dependees(
            variant_defn: &EntityDefn,
            _builder: &mut DependeeMapBuilder,
        ) {
            match variant_defn.variant {
                EntityDefnVariant::EnumVariant {
                    enum_variant_defn_variant: ref variant,
                    ..
                } => match variant {
                    EnumVariantDefnVariant::Constant => (),
                },
                _ => panic!(),
            }
        }

        fn extract_member_dependees(member_defn: &EntityDefn, _builder: &mut DependeeMapBuilder) {
            match member_defn.variant {
                EntityDefnVariant::Module { .. } => unreachable!(),
                EntityDefnVariant::Feature { .. } => todo!(),
                EntityDefnVariant::Func { .. } => todo!(),
                EntityDefnVariant::Proc { .. } => todo!(),
                EntityDefnVariant::Ty { .. } => todo!(),
                EntityDefnVariant::EnumVariant { .. } => todo!(),
                EntityDefnVariant::Builtin => todo!(),
                EntityDefnVariant::TyField { .. } => todo!(),
                EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
                EntityDefnVariant::TraitAssociatedConstSizeImpl { value: _ } => todo!(),
                EntityDefnVariant::Method { .. } => todo!(),
                EntityDefnVariant::Trait { .. } => todo!(),
                EntityDefnVariant::Function { .. } => todo!(),
                EntityDefnVariant::TargetInput { .. } => todo!(),
                EntityDefnVariant::Any => (),
            }
        }

        fn extract_method_dependees(_method_defn: &EntityDefn, _builder: &mut DependeeMapBuilder) {
            todo!()
            // for input_placeholder in method_defn.parameters.iter() {
            //     builder.push(input_placeholder.ranged_ty.route)
            // }
            // builder.push(method_defn.output.route);
            // match method_defn.variant {
            //     MethodDefnVariant::Func { ref stmts } => {
            //         extract_func_stmts_dependees(stmts, builder)
            //     }
            //     MethodDefnVariant::Proc { ref stmts } => todo!(),
            //     MethodDefnVariant::Pattern { ref stmts } => todo!(),
            // }
        }

        fn extract_proc_condition_branch_dependees(
            branch: &ProcConditionFlowBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                ProcConditionFlowBranchVariant::If { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                ProcConditionFlowBranchVariant::Elif { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                ProcConditionFlowBranchVariant::Else => (),
            }
            extract_proc_stmts_dependees(&branch.stmts, builder)
        }

        fn extract_func_condition_branch_dependees(
            branch: &FuncConditionFlowBranch,
            builder: &mut DependeeMapBuilder,
        ) {
            match branch.variant {
                FuncConditionFlowBranchVariant::If { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                FuncConditionFlowBranchVariant::Elif { ref condition } => {
                    extract_eager_expr_dependees(condition, builder)
                }
                FuncConditionFlowBranchVariant::Else => todo!(),
            }
            extract_func_stmts_dependees(&branch.stmts, builder)
        }
    }
}
