use crate::*;
use husky_ast::HasAstSheet;
use husky_ast::{AstData, AstSheet};
use husky_entity_tree::{
    expr::r#use::{ParentUseExprData, UseExpr},
    helpers::paths::module_item_syn_node_paths,
    jar::EntityTreeDb,
    node::ItemSynNodePath,
    presheet::{EntityTreePresheet, OnceUseRule, OnceUseRuleIdx},
    region_path::SynNodeRegionPath,
    sheet::EntityTreeSheet,
    symbol::ModuleSymbolContext,
};
use husky_regional_token::{RegionalTokenIdx, RegionalTokenIdxBase};
use husky_sem_expr::{SemExprData, SemExprDb, SemExprIdx, SemExprRegionData, SemaHtmlArgumentExpr};
use husky_sem_opr::prefix::SemaPrefixOpr;
use husky_syn_decl::decl::HasSynNodeDecl;
use husky_syn_defn::*;
use husky_syn_expr::{
    entity_path::{SynPrincipalEntityPathExpr, SynPrincipalEntityPathSynExprIdx},
    *,
};

pub(crate) struct TokenInfoEngine<'a> {
    db: &'a ::salsa::Db,
    module_path: ModulePath,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    item_tree_presheet: &'a EntityTreePresheet,
    item_tree_sheet: &'a EntityTreeSheet,
    module_symbol_context: ModuleSymbolContext<'a>,
    sheet: TokenInfoSheet,
}

impl<'a> TokenInfoEngine<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet_data = &db.token_sheet_data(module_path);
        Ok(Self {
            db,
            module_path,
            token_sheet_data,
            ast_sheet: module_path.ast_sheet(db),
            item_tree_presheet: db.item_syn_tree_presheet(module_path),
            item_tree_sheet: db.item_syn_tree_sheet(module_path),
            sheet: TokenInfoSheet::new(token_sheet_data),
            module_symbol_context: db.module_symbol_context(module_path)?,
        })
    }

    pub(crate) fn visit_all(mut self) -> EntityTreeResult<TokenInfoSheet> {
        self.visit_syn_nodes()?;
        self.visit_once_use_rules();
        Ok(self.sheet)
    }

    fn visit_syn_nodes(&mut self) -> EntityTreeResult<()> {
        for syn_node_path in module_item_syn_node_paths(self.db, self.module_path)
            .iter()
            .copied()
        {
            self.visit_syn_node(syn_node_path)
        }
        Ok(())
    }

    fn visit_once_use_rules(&mut self) {
        for (rule_idx, rule) in self.item_tree_sheet.once_use_rule_indexed_iter() {
            self.visit_once_use_rule(rule, rule_idx);
        }
    }

    fn visit_once_use_rule(&mut self, rule: &OnceUseRule, rule_idx: OnceUseRuleIdx) {
        let use_expr_idx = rule.use_expr_idx();
        let use_expr = &self.item_tree_presheet[use_expr_idx];
        match use_expr {
            UseExpr::All { star_token } => self.sheet.add(
                star_token.token_idx(),
                rule.use_expr_idx(),
                TokenInfoData::UseExprStar,
            ),
            UseExpr::IdentLeaf { ident_token } => self.sheet.add(
                ident_token.token_idx(),
                rule.use_expr_idx(),
                TokenInfoData::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Parent(ParentUseExprData {
                parent_name_token, ..
            }) => self.sheet.add(
                parent_name_token.token_idx(),
                rule.use_expr_idx(),
                TokenInfoData::UseExpr {
                    use_expr_idx,
                    rule_idx,
                    state: rule.state(),
                },
            ),
            UseExpr::Err(_) => (),
            UseExpr::SelfLeaf { self_mod_token: _ } => todo!(),
        }
    }

    fn visit_syn_node(&mut self, syn_node_path: ItemSynNodePath) {
        let db = self.db;
        if let Some(syn_expr_region) = syn_node_path.syn_node_decl(db).syn_expr_region(db) {
            self.visit_expr_region(syn_expr_region)
        }
        if let Some(ItemSynNodeDefn {
            syn_expr_region, ..
        }) = item_syn_node_defn(db, *syn_node_path)
        {
            self.visit_expr_region(syn_expr_region)
        }
        match self.ast_sheet[syn_node_path.decl_ast_idx(db)] {
            AstData::Identifiable {
                ident_token,
                item_kind,
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfoSource::AstIdentifiable,
                TokenInfoData::EntityNode(syn_node_path, item_kind),
            ),
            AstData::TypeVariant { .. } => (),
            AstData::ImplBlock { .. } => (),
            // ad hoc
            AstData::Attr { .. } => (),
            _ => unreachable!(),
        }
    }

    fn visit_expr_region(&mut self, syn_expr_region: SynExprRegion) {
        DeclTokenInfoEngine::new(self, syn_expr_region).visit_all()
    }
}

struct DeclTokenInfoEngine<'a, 'b> {
    db: &'a ::salsa::Db,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    syn_expr_region_data: &'a SynExprRegionData,
    sem_expr_region_data: &'a SemExprRegionData,
    sheet: &'b mut TokenInfoSheet,
    syn_expr_region: ExprRegionLeash,
    regional_token_idx_base: RegionalTokenIdxBase,
}

impl<'a, 'b> DeclTokenInfoEngine<'a, 'b> {
    fn new(
        engine: &'b mut TokenInfoEngine<'a>,
        syn_expr_region: SynExprRegion,
    ) -> DeclTokenInfoEngine<'a, 'b> {
        let syn_expr_region_data = syn_expr_region.data(engine.db);
        let db = engine.db;
        DeclTokenInfoEngine {
            db,
            token_sheet_data: engine.token_sheet_data,
            ast_sheet: engine.ast_sheet,
            sheet: &mut engine.sheet,
            syn_expr_region_data,
            sem_expr_region_data: db.sem_expr_region(syn_expr_region).data(db),
            syn_expr_region: syn_expr_region.into(),
            regional_token_idx_base: match syn_expr_region_data.path() {
                SynNodeRegionPath::CrateDecl(_) => todo!(),
                SynNodeRegionPath::ItemDecl(path) => path.decl_regional_token_idx_base(db),
                SynNodeRegionPath::ItemDefn(path) => {
                    path.defn_regional_token_idx_base(db).expect("todo")
                }
            },
        }
    }

    fn add(
        &mut self,
        regional_token_idx: RegionalTokenIdx,
        src: impl Into<TokenInfoSource>,
        token_info_data: TokenInfoData,
    ) {
        let base = self.regional_token_idx_base;
        self.sheet
            .add(regional_token_idx.token_idx(base), src, token_info_data)
    }

    fn override_add(
        &mut self,
        regional_token_idx: RegionalTokenIdx,
        src: impl Into<TokenInfoSource>,
        token_info_data: TokenInfoData,
    ) {
        let base = self.regional_token_idx_base;
        self.sheet
            .override_add(regional_token_idx.token_idx(base), src, token_info_data)
    }

    fn visit_all(mut self) {
        for (expr_idx, expr) in self.sem_expr_region_data.sem_expr_arena().indexed_iter() {
            if let Ok(sem_expr_data) = expr.data_result() {
                self.visit_expr(expr_idx, sem_expr_data)
            }
        }
        for (item_path_expr_idx, item_path_expr) in self
            .syn_expr_region_data
            .principal_item_path_expr_arena()
            .indexed_iter()
        {
            self.visit_item_path_expr(item_path_expr_idx, item_path_expr)
        }
        for (current_variable_idx, current_variable) in self
            .syn_expr_region_data
            .variable_region()
            .indexed_current_variables()
        {
            self.visit_current_variable(current_variable_idx, current_variable)
        }
    }

    fn visit_expr(&mut self, sem_expr_idx: SemExprIdx, sem_expr_data: &SemExprData) {
        match sem_expr_data {
            SemExprData::CurrentSynSymbol {
                regional_token_idx,
                current_variable_idx,
                current_variable_kind,
                ..
            }
            | SemExprData::FrameVarDecl {
                regional_token_idx,
                frame_var_symbol_idx: current_variable_idx,
                current_variable_kind,
                ..
            } => self.add(
                *regional_token_idx,
                sem_expr_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_variable_idx: *current_variable_idx,
                    current_variable_kind: *current_variable_kind,
                    syn_expr_region: self.syn_expr_region,
                },
            ),
            SemExprData::InheritedSynSymbol {
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
                ..
            } => self.add(
                *regional_token_idx,
                sem_expr_idx,
                TokenInfoData::InheritedSynSymbol {
                    inherited_syn_symbol_idx: *inherited_syn_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    inherited_syn_symbol_kind: *inherited_syn_symbol_kind,
                },
            ),
            SemExprData::SelfType(regional_token_idx) => {
                self.add(*regional_token_idx, sem_expr_idx, TokenInfoData::SelfType)
            }
            SemExprData::SelfValue(regional_token_idx) => {
                self.add(*regional_token_idx, sem_expr_idx, TokenInfoData::SelfValue)
            }
            SemExprData::Field { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sem_expr_idx,
                TokenInfoData::Field,
            ),
            SemExprData::MethodApplication { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sem_expr_idx,
                TokenInfoData::Method,
            ),
            SemExprData::MethodFnCall { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sem_expr_idx,
                TokenInfoData::Method,
            ),
            SemExprData::MethodGnCall { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sem_expr_idx,
                TokenInfoData::Method,
            ),
            SemExprData::At {
                at_regional_token_idx: _,
                place_label_regional_token,
            } => {
                if let Some(_) = place_label_regional_token {
                    todo!()
                }
                // ad hoc
                // self.add(*at_regional_token_idx, TokenInfoData::Method)
            }
            SemExprData::Prefix {
                opr,
                opr_regional_token_idx,
                ..
            } => match opr {
                SemaPrefixOpr::Minus => (),
                SemaPrefixOpr::Not => (),
                SemaPrefixOpr::BitNot => (),
                SemaPrefixOpr::LeashType | SemaPrefixOpr::RefType | SemaPrefixOpr::OptionType => {
                    self.add(
                        *opr_regional_token_idx,
                        sem_expr_idx,
                        TokenInfoData::SemaPrefixTypeOpr,
                    );
                }
            },
            SemExprData::Literal(_, _)
            | SemExprData::PrincipalEntityPath { .. }
            | SemExprData::Binary { .. }
            | SemExprData::Suffix { .. }
            | SemExprData::Unveil { .. }
            | SemExprData::Unwrap { .. }
            | SemExprData::TemplateInstantiation { .. }
            | SemExprData::NewTuple { .. }
            | SemExprData::NewList { .. }
            | SemExprData::Delimitered { .. }
            | SemExprData::Block { .. }
            | SemExprData::Be { .. } => (),
            SemExprData::FunctionApplication { .. } => (),
            SemExprData::MajorItemPathAssocItem { .. } => (),
            SemExprData::AssocItem { .. } => (),
            SemExprData::Index {
                owner: _,
                lbox_regional_token_idx: _,
                index_sem_list_items: _,
                rbox_regional_token_idx: _,
                index_dynamic_dispatch: _,
            } => (),
            SemExprData::CompositionWithList {
                owner: _,
                lbox_regional_token_idx: _,
                items: _indices,
                rbox_regional_token_idx: _,
            } => (),
            SemExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => {
                self.add(
                    *lpar_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::UnitLeftParenthesis,
                );
                self.add(
                    *rpar_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::UnitRightParenthesis,
                );
            }
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident,
                ref arguments,
                empty_html_ket: _,
            } => {
                self.add(
                    function_ident.regional_token_idx(),
                    sem_expr_idx,
                    TokenInfoData::HtmlFunctionIdent,
                );
                for argument in arguments.iter() {
                    match argument {
                        SemaHtmlArgumentExpr::Expanded { property_ident, .. }
                        | SemaHtmlArgumentExpr::Shortened { property_ident, .. } => self.add(
                            property_ident.regional_token_idx(),
                            sem_expr_idx,
                            TokenInfoData::HtmlPropertyIdent,
                        ),
                    }
                }
            }
            &SemExprData::FunctionRitchieCall {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            } => {
                self.add(
                    lpar_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::CallPar,
                );
                self.add(
                    rpar_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::CallPar,
                );
            }
            SemExprData::Ritchie { .. } => (),
            SemExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemExprData::Todo { regional_token_idx } => {
                self.add(*regional_token_idx, sem_expr_idx, TokenInfoData::Todo)
            }
            SemExprData::Unreachable { regional_token_idx } => self.add(
                *regional_token_idx,
                sem_expr_idx,
                TokenInfoData::Unreachable,
            ),
            SemExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::VecFunctorBoxPrefix,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::VecFunctorBoxPrefix,
                )
            }
            SemExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items: _,
                rbox_regional_token_idx,
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::ArrayFunctorBoxPrefix,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::ArrayFunctorBoxPrefix,
                )
            }
            SemExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::BoxColon,
                );
                self.add(
                    *colon_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::BoxColon,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::BoxColon,
                )
            }
            SemExprData::NestedBlock {
                lcurl_regional_token_idx,
                rcurl_regional_token,
                ..
            } => {
                self.add(
                    *lcurl_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::NestedBlockCurl,
                );
                self.add(
                    rcurl_regional_token.regional_token_idx(),
                    sem_expr_idx,
                    TokenInfoData::NestedBlockCurl,
                )
            }
            &SemExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                rvert_regional_token,
                return_ty,
                ..
            } => {
                if let Some(closure_kind_regional_token_idx) = closure_kind_regional_token_idx {
                    todo!()
                }
                self.add(
                    lvert_regional_token_idx,
                    sem_expr_idx,
                    TokenInfoData::ClosureVert,
                );
                self.add(
                    rvert_regional_token.regional_token_idx(),
                    sem_expr_idx,
                    TokenInfoData::ClosureVert,
                );
                if let Some((light_arrow, _, eq)) = return_ty {
                    self.add(
                        light_arrow.regional_token_idx(),
                        sem_expr_idx,
                        TokenInfoData::ClosureLightArrow,
                    );
                    self.add(
                        eq.regional_token_idx(),
                        sem_expr_idx,
                        TokenInfoData::ClosureEq,
                    );
                }
            }
        }
    }

    fn visit_item_path_expr(
        &mut self,
        item_path_expr_idx: SynPrincipalEntityPathSynExprIdx,
        item_path_expr: &SynPrincipalEntityPathExpr,
    ) {
        match item_path_expr {
            &SynPrincipalEntityPathExpr::Root {
                principal_entity_path,
                path_name_token,
                ..
            } => self.add(
                path_name_token.regional_token_idx(),
                TokenInfoSource::SynPrincipalEntityPathExpr(
                    item_path_expr_idx,
                    principal_entity_path,
                ),
                TokenInfoData::Entity(principal_entity_path.into()),
            ),
            &SynPrincipalEntityPathExpr::Subitem {
                path: Ok(principal_entity_path),
                ident_token: Ok(ident_token),
                ..
            } => self.add(
                ident_token.regional_token_idx(),
                TokenInfoSource::SynPrincipalEntityPathExpr(
                    item_path_expr_idx,
                    principal_entity_path,
                ),
                TokenInfoData::Entity((principal_entity_path).into()),
            ),
            SynPrincipalEntityPathExpr::Subitem { .. } => (),
        }
    }

    fn visit_current_variable(
        &mut self,
        current_variable_idx: CurrentVariableIdx,
        current_variable: &CurrentVariableEntry,
    ) {
        let current_variable_kind = current_variable.kind();
        match current_variable_kind {
            CurrentVariableKind::LetVariable {
                pattern_variable_idx: pattern_symbol,
            }
            | CurrentVariableKind::BeVariable {
                pattern_variable_idx: pattern_symbol,
            }
            | CurrentVariableKind::CaseVariable {
                pattern_variable_idx: pattern_symbol,
            }
            | CurrentVariableKind::SimpleParenateParameter {
                pattern_variable_idx: pattern_symbol,
            }
            | CurrentVariableKind::SimpleClosureParameter {
                pattern_variable_idx: pattern_symbol,
            } => match self.syn_expr_region_data[pattern_symbol] {
                PatternVariable::Atom(pattern_idx) => {
                    match self.syn_expr_region_data[pattern_idx] {
                        SynPatternData::Ident {
                            ident_token,
                            symbol_modifier_tokens: _,
                        } => self.override_add(
                            ident_token.regional_token_idx(),
                            pattern_idx,
                            TokenInfoData::CurrentSynSymbol {
                                current_variable_idx: current_variable_idx,
                                syn_expr_region: self.syn_expr_region,
                                current_variable_kind: current_variable_kind,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            },
            CurrentVariableKind::LoopVariable(_) => (),
            CurrentVariableKind::TemplateParameter {
                template_parameter_kind,
            } => match template_parameter_kind {
                CurrentTemplateParameterSynSymbolKind::Type { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    TokenInfoSource::TemplateParameter(current_variable_idx),
                    TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: current_variable_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_variable_kind: current_variable_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Lifetime { label_token } => self.add(
                    label_token.regional_token_idx(),
                    current_variable_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: current_variable_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_variable_kind: current_variable_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Place { label_token } => self.add(
                    label_token.regional_token_idx(),
                    current_variable_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: current_variable_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_variable_kind: current_variable_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Constant { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    current_variable_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_variable_idx: current_variable_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_variable_kind: current_variable_kind,
                    },
                ),
            },
            CurrentVariableKind::VariadicParenateParameter { ident_token } => self.add(
                ident_token.regional_token_idx(),
                current_variable_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_variable_idx: current_variable_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_variable_kind: current_variable_kind,
                },
            ),
            CurrentVariableKind::FieldVariable { ident_token } => self.add(
                ident_token.regional_token_idx(),
                current_variable_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_variable_idx: current_variable_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_variable_kind: current_variable_kind,
                },
            ),
        }
    }
}
