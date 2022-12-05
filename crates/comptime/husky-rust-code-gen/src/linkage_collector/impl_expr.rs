use super::*;
use husky_eager_semantics::{EagerExpr, EagerExprVariant, EagerOpnVariant};
use husky_entity_kind::FieldKind;
use husky_lazy_semantics::{LazyExpr, LazyExprVariant, LazyOpnKind};

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_from_eager_expr(&mut self, expr: &EagerExpr) {
        match expr.variant {
            EagerExprVariant::Variable { .. } => (),
            EagerExprVariant::ThisValue { .. } => (),
            EagerExprVariant::ThisField { .. } => (),
            EagerExprVariant::PrimitiveLiteral(_) => (),
            EagerExprVariant::EnumKindLiteral(_) => self.insert(expr.intrinsic_ty()),
            EagerExprVariant::Bracketed(ref expr) => self.collect_from_eager_expr(expr),
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => {
                for opd in opds {
                    self.collect_from_eager_expr(opd);
                }
                match opn_variant {
                    EagerOpnVariant::Binary { .. } => (),
                    EagerOpnVariant::Prefix { .. } => (),
                    EagerOpnVariant::Suffix { .. } => (),
                    EagerOpnVariant::RoutineCall(routine) => self.insert(routine.route),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => self.insert(ranged_ty.route),
                    EagerOpnVariant::Field {
                        field_kind,
                        field_ident,
                        ..
                    } => match field_kind {
                        FieldKind::StructRegular
                        | FieldKind::StructDefault
                        | FieldKind::StructDerived => (),
                        FieldKind::StructMemo => self.insert(self.db.subroute(
                            opds[0].intrinsic_ty(),
                            field_ident.ident,
                            Default::default(),
                        )),
                        FieldKind::RecordRegular => todo!(),
                        FieldKind::RecordProperty => todo!(),
                    },
                    EagerOpnVariant::MethodCall { method_route, .. } => {
                        match method_route.variant {
                            EntityRouteVariant::TypeAsTraitMember { .. } => {
                                self.insert(*method_route)
                            }
                            _ => self.insert(*method_route),
                        }
                    }
                    EagerOpnVariant::Index { .. } => (),
                    EagerOpnVariant::NewVecFromList => self.insert(expr.intrinsic_ty()),
                    EagerOpnVariant::ValueCall => self.insert(opds[0].intrinsic_ty()),
                }
            }
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::EntityFeature { route } => self.insert(route),
            EagerExprVariant::EntityThickFp { route } => self.insert(route),
        }
    }

    pub(crate) fn collect_from_lazy_expr(&mut self, expr: &LazyExpr) {
        match expr.variant {
            LazyExprVariant::Variable { .. } => (),
            LazyExprVariant::PrimitiveLiteral(_) => (),
            LazyExprVariant::EnumLiteral { entity_route } => self.insert(entity_route.parent()),
            LazyExprVariant::Bracketed(ref bracketed_expr) => {
                self.collect_from_lazy_expr(bracketed_expr)
            }
            LazyExprVariant::Opn { opn_kind, ref opds } => {
                for opd in opds {
                    self.collect_from_lazy_expr(opd);
                }
                match opn_kind {
                    LazyOpnKind::Binary { .. } => (),
                    LazyOpnKind::Prefix(_) => (),
                    LazyOpnKind::FunctionModelCall(ranged_route) => self.insert(ranged_route.route),
                    LazyOpnKind::FunctionRoutineCall(ranged_route) => {
                        self.insert(ranged_route.route)
                    }
                    LazyOpnKind::StructCall(_) => todo!(),
                    LazyOpnKind::NewVecFromList => self.insert(expr.intrinsic_ty()),
                    LazyOpnKind::RecordCall(ranged_route) => self.insert(ranged_route.route),
                    LazyOpnKind::Field { .. } => (),
                    LazyOpnKind::MethodCall { method_route, .. } => self.insert(method_route),
                    LazyOpnKind::Index { .. } => (),
                }
            }
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::ThisValue { .. } => todo!(),
            LazyExprVariant::ThisField { .. } => todo!(),
            LazyExprVariant::EntityFeature { entity_route } => self.insert(entity_route),
            LazyExprVariant::BePattern { .. } => (),
        }
    }
}
