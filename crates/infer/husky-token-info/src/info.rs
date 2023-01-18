use husky_entity_path::EntityPath;
use husky_entity_taxonomy::EntityKind;
use husky_entity_tree::{UseExprIdx, UseExprRuleIdx, UseExprRuleState};
use husky_expr::{
    ExprPage, InheritedSymbolIdx, InheritedSymbolKind, LocalSymbolIdx, LocalSymbolKind,
};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfo {
    None,
    Entity(Option<EntityPath>, Option<EntityKind>),
    InheritedSymbol {
        inherited_symbol_idx: InheritedSymbolIdx,
        inherited_symbol_kind: InheritedSymbolKind,
        expr_page: ExprPage,
    },
    LocalSymbol {
        local_symbol_idx: LocalSymbolIdx,
        local_symbol_kind: LocalSymbolKind,
        expr_page: ExprPage,
    },
    SelfType,
    SelfValue,
    Field,
    Method,
    BoxColon,
    BoxPrefix,
    UseExprStar,
    UseExpr {
        use_expr_idx: UseExprIdx,
        rule_idx: UseExprRuleIdx,
        state: UseExprRuleState,
    },
}

impl Default for TokenInfo {
    fn default() -> Self {
        TokenInfo::None
    }
}
