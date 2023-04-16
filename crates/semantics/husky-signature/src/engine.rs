mod pattern_ty;

pub(crate) use self::pattern_ty::*;

use crate::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use salsa::DebugWithDb;

pub(super) struct DeclarativeTermEngine<'a> {
    db: &'a dyn SignatureDb,
    expr_region_data: &'a ExprRegionData,
    raw_term_menu: &'a DeclarativeTermMenu,
    raw_term_symbol_region: SymbolDeclarativeTermRegion,
    expr_terms: ExprMap<SignatureDeclarativeTermResult<DeclarativeTerm>>,
    /// todo: change this to ordered
    pattern_expr_ty_infos: PatternExprMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: PatternSymbolMap<PatternSymbolTypeInfo>,
}

#[salsa::tracked(jar = SignatureJar, return_ref)]
pub(crate) fn signature_term_region(
    db: &dyn SignatureDb,
    expr_region: ExprRegion,
) -> SignatureRegion {
    let expr_region_data = expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| signature_term_region(db, r).term_symbol_region());
    let mut engine = DeclarativeTermEngine::new(db, expr_region, parent_term_symbol_region);
    engine.infer_all()
}

impl<'a> DeclarativeTermEngine<'a> {
    fn new(
        db: &'a dyn SignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a SymbolDeclarativeTermRegion>,
    ) -> Self {
        let toolchain = expr_region.toolchain(db);
        // ad hoc
        let _entity_path_menu = db.entity_path_menu(toolchain);
        let raw_term_menu = db.raw_term_menu(toolchain).unwrap();
        let expr_region_data = &expr_region.data(db);
        Self {
            db,
            expr_region_data,
            raw_term_menu,
            raw_term_symbol_region: SymbolDeclarativeTermRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            pattern_expr_ty_infos: PatternExprMap::new(expr_region_data.pattern_expr_arena()),
            pattern_symbol_ty_infos: PatternSymbolMap::new(
                expr_region_data
                    .pattern_expr_region()
                    .pattern_symbol_arena(),
            ),
        }
    }

    fn infer_all(mut self) -> SignatureRegion {
        self.init_current_symbol_terms();
        self.raw_term_symbol_region.init_self_ty_and_value(
            self.db,
            self.expr_region_data.path(),
            self.expr_region_data.symbol_region(),
        );
        self.init_expr_roots();
        self.finish()
    }

    fn init_current_symbol_terms(&mut self) {
        let mut current_symbol_indexed_iter = self
            .expr_region_data
            .symbol_region()
            .current_symbol_indexed_iter();
        for (pattern_ty_constraint, symbols) in self
            .expr_region_data
            .symbol_region()
            .pattern_ty_constraints()
        {
            match pattern_ty_constraint {
                PatternTypeConstraint::ImplicitTypeParameter => {
                    let (current_symbol_idx, current_symbol) = current_symbol_indexed_iter
                        .next()
                        .expect("ty constraint should match with current symbols");
                    let CurrentSymbolVariant::ImplicitParameter {
                        implicit_parameter_variant,
                    } = current_symbol.variant() else {
                        unreachable!()
                    };
                    let ty = match implicit_parameter_variant {
                        CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                            Ok(self.raw_term_menu.lifetime_ty().into())
                        }
                        CurrentImplicitParameterSymbol::Type { ident_token } => {
                            Ok(self.raw_term_menu.ty0().into())
                        }
                        _ => todo!(),
                    };
                    self.raw_term_symbol_region
                        .add_new_implicit_parameter_symbol_signature(self.db, symbols.start(), ty)
                }
                PatternTypeConstraint::ExplicitParameter { pattern_expr, ty } => self
                    .init_current_symbol_signatures_in_explicit_parameter(
                        *pattern_expr,
                        *ty,
                        *symbols,
                    ),
                PatternTypeConstraint::LetVariables { .. }
                | PatternTypeConstraint::FrameVariable => {
                    // need only to compute for decl region
                    return;
                }
            }
        }
    }

    /// explicit parameters are infered in this crate;
    ///
    /// let variables, be variables and match variables are infered in `husky-expr-ty`
    fn init_current_symbol_signatures_in_explicit_parameter(
        &mut self,
        pattern_expr: PatternExprIdx,
        ty: ExprIdx,
        symbols: CurrentSymbolIdxRange,
    ) {
        let Ok(ty) = self.infer_new_expr_term(ty) else {
            for symbol in symbols {
                let modifier = self.expr_region_data[symbol].modifier();
                self.raw_term_symbol_region.add_new_explicit_parameter_symbol_signature(
                    self.db,
                    symbol,
                    modifier,
                    Err(DeclarativeTermSymbolTypeErrorKind::SignatureDeclarativeTermError),
                )
            }
            return
        };
        self.infer_pattern_tys_in_explicit_parameter(pattern_expr, ty);
        for symbol in symbols {
            self.infer_current_symbol_signature_in_explicit_parameter(symbol)
        }
    }

    fn infer_current_symbol_signature_in_explicit_parameter(
        &mut self,
        current_symbol_idx: CurrentSymbolIdx,
    ) {
        let current_symbol = &self.expr_region_data.symbol_region()[current_symbol_idx];
        match current_symbol.variant() {
            CurrentSymbolVariant::ExplicitParameter {
                ident,
                pattern_symbol_idx,
            } => {
                let base_ty = self.pattern_symbol_ty_infos[pattern_symbol_idx].base_ty();
                self.raw_term_symbol_region
                    .add_new_explicit_parameter_symbol_signature(
                        self.db,
                        current_symbol_idx,
                        current_symbol.modifier(),
                        Ok(base_ty),
                    )
            }
            _ => unreachable!("this function is only used for explicit parameters"),
        }
    }

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::BlockExpr => return,
                ExprRootKind::SelfType
                | ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::FieldType
                | ExprRootKind::VarType => (),
            }
            self.cache_new_expr_term(expr_root.expr())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new_expr_term(&mut self, expr_idx: ExprIdx) -> SignatureDeclarativeTermResult<DeclarativeTerm> {
        let result = self.calc_expr_term(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedSignatureDeclarativeTermError::DeclarativeTermAbortion.into()),
        };
        self.save_expr_term(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new_expr_term(&mut self, expr_idx: ExprIdx) {
        let result = self.calc_expr_term(expr_idx);
        self.save_expr_term(expr_idx, result)
    }

    pub(crate) fn finish(self) -> SignatureRegion {
        SignatureRegion::new(
            self.expr_region_data.path(),
            self.raw_term_symbol_region,
            self.expr_terms,
            self.pattern_expr_ty_infos,
            self.pattern_symbol_ty_infos,
        )
    }

    fn save_expr_term(&mut self, expr_idx: ExprIdx, outcome: SignatureDeclarativeTermResult<DeclarativeTerm>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc_expr_term(&mut self, expr_idx: ExprIdx) -> SignatureDeclarativeTermResult<DeclarativeTerm> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr: _,
                path: entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(DeclarativeTerm::EntityPath(match entity_path {
                    EntityPath::Module(_) => todo!(),
                    EntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(path) => {
                            /* ad hoc */
                            DeclarativeTermEntityPath::Type(path)
                        }
                        ModuleItemPath::Trait(path) => path.into(),
                        ModuleItemPath::Form(path) => path.into(),
                    },
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::TypeVariant(_) => todo!(),
                })),
                None => Err(DerivedSignatureDeclarativeTermError::InvalidEntityPath.into()),
            },
            Expr::InheritedSymbol {
                ident: _,
                token_idx: _,
                inherited_symbol_idx: _,
                inherited_symbol_kind: _,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .raw_term_symbol_region
                .current_symbol_signature(current_symbol_idx)
                .expect("not none")
                .symbol()
                .ok_or(OriginalSignatureDeclarativeTermError::InvalidSymbolForTerm)?
                .into()),
            Expr::FrameVarDecl { .. } => unreachable!(),
            Expr::SelfType(_) => self
                .raw_term_symbol_region
                .self_ty_term()
                .ok_or(DerivedSignatureDeclarativeTermError::SelfTypeNotAllowedInThisRegion.into()),
            Expr::SelfValue(_) => self
                .raw_term_symbol_region
                .self_ty_term()
                .ok_or(DerivedSignatureDeclarativeTermError::SelfValueNotAllowedInThisRegion.into()),
            Expr::Binary {
                lopd, opr, ropd, ..
            } => {
                let Ok(lopd) = self.infer_new_expr_term(lopd) else {
                    return Err(DerivedSignatureDeclarativeTermError::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                let Ok(ropd) = self.infer_new_expr_term(ropd) else {
                    return Err(DerivedSignatureDeclarativeTermError::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                match opr {
                    BinaryOpr::Closed(_) => todo!(),
                    BinaryOpr::Shift(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortCircuitLogic(_) => todo!(),
                    BinaryOpr::Assign => todo!(),
                    BinaryOpr::AssignClosed(_) => todo!(),
                    BinaryOpr::AssignShift(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Ok(DeclarativeTermCurry::new(
                        self.db,
                        CurryKind::Explicit, // ad hoc
                        Variance::Invariant, // ad hoc
                        None,                // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Ins => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx: _,
                opd,
            } => {
                let Ok(opd) = self.infer_new_expr_term(opd) else {
                    return Err(DerivedSignatureDeclarativeTermError::CannotInferOperandDeclarativeTermInPrefix.into());
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::Tilde => {
                        DeclarativeTerm::LeashOrBitNot(self.expr_region_data.path().toolchain(self.db))
                    }
                    PrefixOpr::Ref => self.raw_term_menu.ref_ty_path(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.raw_term_menu.option_ty_path(),
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, tmpl, opd).into())
            }
            Expr::Suffix {
                opd: _,
                opr: _punctuation,
                opr_token_idx: _punctuation_token_idx,
            } => todo!(),
            Expr::Field {
                owner: _self_expr,
                dot_token_idx: _,
                ident_token: _,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ExplicitApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                items,
                ref commas,
                ..
            } => {
                let Ok(function) = self.infer_new_expr_term(function) else {
                    return Err(
                        DerivedSignatureDeclarativeTermError::CannotInferArgumentDeclarativeTermInApplication.into()
                    )
                };
                let implicit_arguments = match implicit_arguments {
                    Some(implicit_arguments) => implicit_arguments
                        .arguments()
                        .into_iter()
                        .map(|_| todo!())
                        .collect(),
                    None => vec![],
                };
                assert!(items.len() <= commas.len() + 1);
                assert!(items.len() >= commas.len());
                let extra_comma = items.len() == commas.len();
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item))
                    .collect::<SignatureDeclarativeTermResult<_>>()?;
                Ok(DeclarativeTermExplicitApplicationOrRitchieCall::new(
                    self.db,
                    function,
                    implicit_arguments,
                    items,
                    extra_comma,
                )
                .into())
            }
            Expr::ExplicitApplication { function, argument } => {
                let Ok(argument) = self.infer_new_expr_term(argument) else {
                    Err(DerivedSignatureDeclarativeTermError::CannotInferArgumentDeclarativeTermInApplication)?
                };
                let Ok( function) = self.infer_new_expr_term(function) else {
                    Err(DerivedSignatureDeclarativeTermError::CannotInferFunctionDeclarativeTermInApplication)?
                };
                Ok(DeclarativeTermExplicitApplication::new(self.db, function, argument).into())
            }
            Expr::NewTuple { items, .. } => {
                p!(self.expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            Expr::List { items, .. } => {
                let items = items
                    .into_iter()
                    .map(|item| self.infer_new_expr_term(item))
                    .collect::<SignatureDeclarativeTermResult<Vec<_>>>()?;
                Ok(DeclarativeTermList::new(
                    self.db,
                    self.expr_region_data.path().toolchain(self.db),
                    items,
                )
                .into())
            }
            Expr::BoxColonList { items, .. } => match items.len() {
                0 => Ok(self.raw_term_menu.slice_ty_path()),
                _ => todo!(),
            },
            Expr::Bracketed { item, .. } => self.infer_new_expr_term(item),
            Expr::Block { stmts: _ } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner: _,
                lbox_token_idx: _,
                items: _indices,
                rbox_token_idx: _,
            } => todo!(),
            Expr::Err(_) => Err(DerivedSignatureDeclarativeTermError::ExprError.into()),
            Expr::Unit {
                lpar_token_idx,
                rpar_token_idx,
            } => todo!(),
        }
    }

    pub(crate) fn current_symbol_term(&self, symbol: CurrentSymbolIdx) -> Option<SymbolSignature> {
        self.raw_term_symbol_region.current_symbol_signature(symbol)
    }

    pub(crate) fn raw_term_menu(&self) -> &DeclarativeTermMenu {
        self.raw_term_menu
    }
}
