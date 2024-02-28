use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_path::{EntityPath, PrincipalEntityPath};
use husky_entity_tree::{OnceUseRuleIdx, UseExprIdx, UseOneRuleState};
use husky_sema_expr::SemaExprIdx;
use husky_syn_expr::{
    entity_path::PrincipalEntityPathSynExprIdx, CurrentSynSymbolIdx, CurrentSynSymbolKind,
    InheritedSynSymbolIdx, InheritedSynSymbolKind, PatternSynExprIdx, SynExprRegion,
};
#[cfg(feature = "protocol_support")]
use husky_token_protocol::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct TokenInfo {
    src: TokenInfoSource,
    data: TokenInfoData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TokenInfoSource {
    UseExpr(UseExprIdx),
    SemaExpr(SemaExprIdx),
    SynPrincipalEntityPathExpr(PrincipalEntityPathSynExprIdx, PrincipalEntityPath),
    PatternExpr(PatternSynExprIdx),
    // todo: add #[skip] attribute
    TemplateParameter(CurrentSynSymbolIdx),
    AstIdentifiable,
}

impl TokenInfo {
    pub fn new(src: impl Into<TokenInfoSource>, data: TokenInfoData) -> Self {
        Self {
            src: src.into(),
            data,
        }
    }

    pub fn src(&self) -> TokenInfoSource {
        self.src
    }

    pub fn data(&self) -> &TokenInfoData {
        &self.data
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum TokenInfoData {
    Entity(EntityPath),
    EntityNode(ItemSynNodePath, EntityKind),
    InheritedSynSymbol {
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
        syn_expr_region: ExprRegionLeash,
    },
    CurrentSynSymbol {
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
        syn_expr_region: ExprRegionLeash,
    },
    SelfType,
    SelfValue,
    Field,
    Method,
    BoxColon,
    VecFunctorBoxPrefix,
    ArrayFunctorBoxPrefix,
    UseExprStar,
    UseExpr {
        use_expr_idx: UseExprIdx,
        rule_idx: OnceUseRuleIdx,
        state: UseOneRuleState,
    },
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    UnitLeftParenthesis,
    UnitRightParenthesis,
    Todo,
    Unreachable,
    SemaPrefixTypeOpr,
    CallPar,
    NestedBlockCurl,
    ClosureVert,
    ClosureLightArrow,
    ClosureEq,
}

#[cfg(feature = "protocol_support")]
impl TokenInfoData {
    pub fn token_class(&self, db: &::salsa::Db) -> TokenClass {
        match self {
            TokenInfoData::Entity(path) => path.item_kind(db).class().into(),
            TokenInfoData::EntityNode(_path, item_kind) => item_kind.class().into(),
            TokenInfoData::CurrentSynSymbol {
                current_syn_symbol_kind,
                ..
            } => match current_syn_symbol_kind {
                CurrentSynSymbolKind::LetVariable { .. }
                | CurrentSynSymbolKind::BeVariable { .. }
                | CurrentSynSymbolKind::CaseVariable { .. } => TokenClass::Variable,
                CurrentSynSymbolKind::SimpleParenateParameter { .. }
                | CurrentSynSymbolKind::VariadicParenateParameter { .. }
                | CurrentSynSymbolKind::SimpleClosureParameter { .. } => TokenClass::Parameter,
                CurrentSynSymbolKind::LoopVariable(_) => TokenClass::LoopVariable,
                CurrentSynSymbolKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                CurrentSynSymbolKind::FieldVariable { .. } => TokenClass::Variable,
            },
            // TokenProtocol::Variable,
            TokenInfoData::InheritedSynSymbol {
                inherited_syn_symbol_kind,
                ..
            } => match inherited_syn_symbol_kind {
                InheritedSynSymbolKind::ParenateParameter { .. } => TokenClass::Parameter,
                InheritedSynSymbolKind::TemplateParameter { .. } => TokenClass::ImplicitParameter,
                InheritedSynSymbolKind::FieldVariable { .. } => TokenClass::Variable,
            },
            TokenInfoData::SelfType => TokenClass::SelfType,
            TokenInfoData::SelfValue => TokenClass::SelfValue,
            // TokenProtocol::Variable,
            TokenInfoData::Field => TokenClass::Field,
            TokenInfoData::Method => TokenClass::Method,
            TokenInfoData::BoxColon
            | TokenInfoData::VecFunctorBoxPrefix
            | TokenInfoData::ArrayFunctorBoxPrefix => TokenClass::TypeEntity,
            TokenInfoData::UseExpr { state, .. } => match state {
                UseOneRuleState::Resolved {
                    original_symbol: Some(original_symbol),
                } => original_symbol
                    .principal_entity_path(db)
                    .item_kind(db)
                    .class()
                    .into(),
                UseOneRuleState::Resolved {
                    original_symbol: None,
                } => todo!(),
                UseOneRuleState::Unresolved => todo!(),
                UseOneRuleState::Erroneous => TokenClass::Error,
            },
            TokenInfoData::UseExprStar => TokenClass::Punctuation,
            TokenInfoData::HtmlFunctionIdent => TokenClass::HtmlFunctionIdent,
            TokenInfoData::HtmlPropertyIdent => TokenClass::HtmlPropertyIdent,
            TokenInfoData::UnitLeftParenthesis | TokenInfoData::UnitRightParenthesis => {
                TokenClass::TypeEntity
            }
            TokenInfoData::Todo => TokenClass::Todo,
            TokenInfoData::Unreachable => TokenClass::Unreachable,
            TokenInfoData::SemaPrefixTypeOpr => TokenClass::TypeEntity,
            TokenInfoData::CallPar => TokenClass::Punctuation,
            TokenInfoData::NestedBlockCurl => TokenClass::Punctuation,
            TokenInfoData::ClosureVert => TokenClass::Punctuation,
            TokenInfoData::ClosureLightArrow => TokenClass::Punctuation,
            TokenInfoData::ClosureEq => TokenClass::Punctuation,
        }
    }
}

/// the purpose is to avoid extra debug with db in expr region
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprRegionLeash(SynExprRegion);

impl From<SynExprRegion> for ExprRegionLeash {
    fn from(value: SynExprRegion) -> Self {
        ExprRegionLeash(value)
    }
}
impl From<ExprRegionLeash> for SynExprRegion {
    fn from(value: ExprRegionLeash) -> Self {
        value.0
    }
}

impl std::ops::Deref for ExprRegionLeash {
    type Target = SynExprRegion;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for ExprRegionLeash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}

impl salsa::DebugWithDb for ExprRegionLeash {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("ExprRegionLeash(_)")
    }
}
