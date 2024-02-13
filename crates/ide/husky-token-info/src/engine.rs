use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_regional_token::{RegionalTokenIdx, RegionalTokenIdxBase};
use husky_sema_opr::prefix::SemaPrefixOpr;
use husky_syn_decl::decl::HasSynNodeDecl;
use husky_syn_defn::*;

use husky_ast::HasAstSheet;
use husky_entity_tree::{helpers::paths::module_item_syn_node_paths, ParentUseExprData};
use husky_sema_expr::{
    SemaExprData, SemaExprDb, SemaExprIdx, SemaExprRegionData, SemaHtmlArgumentExpr,
};
use husky_syn_expr::{
    entity_path::{PrincipalEntityPathSynExprIdx, SynPrincipalEntityPathExpr},
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
            Ast::Identifiable {
                ident_token,
                item_kind,
                ..
            } => self.sheet.add(
                ident_token.token_idx(),
                TokenInfoSource::AstIdentifiable,
                TokenInfoData::EntityNode(syn_node_path, item_kind),
            ),
            Ast::ImplBlock { .. } => (),
            // ad hoc
            Ast::Attr { .. } => (),
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
    sema_expr_region_data: &'a SemaExprRegionData,
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
            sema_expr_region_data: db.sema_expr_region(syn_expr_region).data(db),
            syn_expr_region: syn_expr_region.into(),
            regional_token_idx_base: match syn_expr_region_data.path() {
                SynNodeRegionPath::Snippet(_) => todo!(),
                SynNodeRegionPath::Decl(path) => path.decl_regional_token_idx_base(db),
                SynNodeRegionPath::Defn(path) => {
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
        for (expr_idx, expr) in self.sema_expr_region_data.sema_expr_arena().indexed_iter() {
            if let Ok(sema_expr_data) = expr.data_result() {
                self.visit_expr(expr_idx, sema_expr_data)
            }
        }
        for (item_path_expr_idx, item_path_expr) in self
            .syn_expr_region_data
            .principal_item_path_expr_arena()
            .indexed_iter()
        {
            self.visit_item_path_expr(item_path_expr_idx, item_path_expr)
        }
        for (current_syn_symbol_idx, current_syn_symbol) in self
            .syn_expr_region_data
            .symbol_region()
            .indexed_current_syn_symbols()
        {
            self.visit_current_syn_symbol(current_syn_symbol_idx, current_syn_symbol)
        }
    }

    fn visit_expr(&mut self, sema_expr_idx: SemaExprIdx, sema_expr_data: &SemaExprData) {
        match sema_expr_data {
            SemaExprData::CurrentSynSymbol {
                regional_token_idx,
                current_syn_symbol_idx,
                current_syn_symbol_kind,
                ..
            }
            | SemaExprData::FrameVarDecl {
                regional_token_idx,
                frame_var_symbol_idx: current_syn_symbol_idx,
                current_syn_symbol_kind,
                ..
            } => self.add(
                *regional_token_idx,
                sema_expr_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_syn_symbol_idx: *current_syn_symbol_idx,
                    current_syn_symbol_kind: *current_syn_symbol_kind,
                    syn_expr_region: self.syn_expr_region,
                },
            ),
            SemaExprData::InheritedSynSymbol {
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
                ..
            } => self.add(
                *regional_token_idx,
                sema_expr_idx,
                TokenInfoData::InheritedSynSymbol {
                    inherited_syn_symbol_idx: *inherited_syn_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    inherited_syn_symbol_kind: *inherited_syn_symbol_kind,
                },
            ),
            SemaExprData::SelfType(regional_token_idx) => {
                self.add(*regional_token_idx, sema_expr_idx, TokenInfoData::SelfType)
            }
            SemaExprData::SelfValue(regional_token_idx) => {
                self.add(*regional_token_idx, sema_expr_idx, TokenInfoData::SelfValue)
            }
            SemaExprData::Field { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sema_expr_idx,
                TokenInfoData::Field,
            ),
            SemaExprData::MethodApplication { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sema_expr_idx,
                TokenInfoData::Method,
            ),
            SemaExprData::MethodFnCall { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sema_expr_idx,
                TokenInfoData::Method,
            ),
            SemaExprData::MethodGnCall { ident_token, .. } => self.add(
                ident_token.regional_token_idx(),
                sema_expr_idx,
                TokenInfoData::Method,
            ),
            SemaExprData::At {
                at_regional_token_idx: _,
                place_label_regional_token,
            } => {
                if let Some(_) = place_label_regional_token {
                    todo!()
                }
                // ad hoc
                // self.add(*at_regional_token_idx, TokenInfoData::Method)
            }
            SemaExprData::Prefix {
                opr,
                opr_regional_token_idx,
                ..
            } => match opr {
                SemaPrefixOpr::Minus => (),
                SemaPrefixOpr::Not => (),
                SemaPrefixOpr::BitNot => (),
                SemaPrefixOpr::LeashType | SemaPrefixOpr::RefType | SemaPrefixOpr::Option => {
                    self.add(
                        *opr_regional_token_idx,
                        sema_expr_idx,
                        TokenInfoData::SemaPrefixTypeOpr,
                    );
                }
            },
            SemaExprData::Literal(_, _)
            | SemaExprData::PrincipalEntityPath { .. }
            | SemaExprData::AssocItem { .. }
            | SemaExprData::Binary { .. }
            | SemaExprData::Suffix { .. }
            | SemaExprData::Unveil { .. }
            | SemaExprData::Unwrap { .. }
            | SemaExprData::TemplateInstantiation { .. }
            | SemaExprData::NewTuple { .. }
            | SemaExprData::NewList { .. }
            | SemaExprData::Delimitered { .. }
            | SemaExprData::Block { .. }
            | SemaExprData::Be { .. } => (),
            SemaExprData::FunctionApplication { .. } => (),
            SemaExprData::Index {
                owner_sema_expr_idx: _,
                lbox_regional_token_idx: _,
                index_sema_list_items: _,
                rbox_regional_token_idx: _,
                index_dynamic_dispatch: _,
            } => (),
            SemaExprData::CompositionWithList {
                owner: _,
                lbox_regional_token_idx: _,
                items: _indices,
                rbox_regional_token_idx: _,
            } => (),
            SemaExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => {
                self.add(
                    *lpar_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::UnitLeftParenthesis,
                );
                self.add(
                    *rpar_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::UnitRightParenthesis,
                );
            }
            SemaExprData::EmptyHtmlTag {
                empty_html_bra_idx: _,
                function_ident,
                ref arguments,
                empty_html_ket: _,
            } => {
                self.add(
                    function_ident.regional_token_idx(),
                    sema_expr_idx,
                    TokenInfoData::HtmlFunctionIdent,
                );
                for argument in arguments.iter() {
                    match argument {
                        SemaHtmlArgumentExpr::Expanded { property_ident, .. }
                        | SemaHtmlArgumentExpr::Shortened { property_ident, .. } => self.add(
                            property_ident.regional_token_idx(),
                            sema_expr_idx,
                            TokenInfoData::HtmlPropertyIdent,
                        ),
                    }
                }
            }
            &SemaExprData::FunctionRitchieCall {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
                ..
            } => {
                self.add(
                    lpar_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::CallPar,
                );
                self.add(
                    rpar_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::CallPar,
                );
            }
            SemaExprData::Ritchie { .. } => (),
            SemaExprData::Sorry {
                regional_token_idx: _,
            } => todo!(),
            SemaExprData::Todo { regional_token_idx } => {
                self.add(*regional_token_idx, sema_expr_idx, TokenInfoData::Todo)
            }
            SemaExprData::Unreachable { regional_token_idx } => self.add(
                *regional_token_idx,
                sema_expr_idx,
                TokenInfoData::Unreachable,
            ),
            SemaExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::VecFunctorBoxPrefix,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::VecFunctorBoxPrefix,
                )
            }
            SemaExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items: _,
                rbox_regional_token_idx,
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::ArrayFunctorBoxPrefix,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::ArrayFunctorBoxPrefix,
                )
            }
            SemaExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                rbox_regional_token_idx,
                ..
            } => {
                self.add(
                    *lbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::BoxColon,
                );
                self.add(
                    *colon_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::BoxColon,
                );
                self.add(
                    *rbox_regional_token_idx,
                    sema_expr_idx,
                    TokenInfoData::BoxColon,
                )
            }
        }
    }

    fn visit_item_path_expr(
        &mut self,
        item_path_expr_idx: PrincipalEntityPathSynExprIdx,
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

    fn visit_current_syn_symbol(
        &mut self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol: &CurrentSynSymbol,
    ) {
        let current_syn_symbol_kind = current_syn_symbol.kind();
        match current_syn_symbol_kind {
            CurrentSynSymbolKind::LetVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | CurrentSynSymbolKind::BeVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | CurrentSynSymbolKind::CaseVariable {
                pattern_symbol_idx: pattern_symbol,
            }
            | CurrentSynSymbolKind::ParenateRegularParameter {
                pattern_symbol_idx: pattern_symbol,
            } => match self.syn_expr_region_data[pattern_symbol] {
                SynPatternSymbol::Atom(pattern_expr_idx) => {
                    match self.syn_expr_region_data[pattern_expr_idx] {
                        SynPatternExprData::Ident {
                            ident_token,
                            symbol_modifier_tokens: _,
                        } => self.override_add(
                            ident_token.regional_token_idx(),
                            pattern_expr_idx,
                            TokenInfoData::CurrentSynSymbol {
                                current_syn_symbol_idx: current_syn_symbol_idx,
                                syn_expr_region: self.syn_expr_region,
                                current_syn_symbol_kind,
                            },
                        ),
                        _ => unreachable!(),
                    }
                }
            },
            CurrentSynSymbolKind::LoopVariable(_) => (),
            CurrentSynSymbolKind::TemplateParameter {
                template_parameter_kind,
            } => match template_parameter_kind {
                CurrentTemplateParameterSynSymbolKind::Type { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    TokenInfoSource::TemplateParameter(current_syn_symbol_idx),
                    TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: current_syn_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_syn_symbol_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Lifetime { label_token } => self.add(
                    label_token.regional_token_idx(),
                    current_syn_symbol_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: current_syn_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_syn_symbol_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Place { label_token } => self.add(
                    label_token.regional_token_idx(),
                    current_syn_symbol_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: current_syn_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_syn_symbol_kind,
                    },
                ),
                CurrentTemplateParameterSynSymbolKind::Constant { ident_token } => self.add(
                    ident_token.regional_token_idx(),
                    current_syn_symbol_idx,
                    TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: current_syn_symbol_idx,
                        syn_expr_region: self.syn_expr_region,
                        current_syn_symbol_kind,
                    },
                ),
            },
            CurrentSynSymbolKind::ParenateVariadicParameter { ident_token } => self.add(
                ident_token.regional_token_idx(),
                current_syn_symbol_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_syn_symbol_idx: current_syn_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_syn_symbol_kind,
                },
            ),
            CurrentSynSymbolKind::FieldVariable { ident_token } => self.add(
                ident_token.regional_token_idx(),
                current_syn_symbol_idx,
                TokenInfoData::CurrentSynSymbol {
                    current_syn_symbol_idx: current_syn_symbol_idx,
                    syn_expr_region: self.syn_expr_region,
                    current_syn_symbol_kind,
                },
            ),
        }
    }
}
