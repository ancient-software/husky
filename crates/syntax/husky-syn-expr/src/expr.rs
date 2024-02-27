mod closure;
mod html;
mod list_item;

pub use self::html::*;
pub use self::list_item::*;

use crate::{closure_parameter::ClosureParameterSyndicate, *};
use husky_term_prelude::ritchie::RitchieKind;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};
use parsec::PunctuatedSmallList;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SynExprData {
    Literal(RegionalTokenIdx, LiteralTokenData),
    PrincipalEntityPath {
        path_expr_idx: PrincipalEntityPathSynExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssocItem {
        parent_expr_idx: PrincipalEntityPathSynExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
    InheritedSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
        inherited_syn_symbol_kind: InheritedSynSymbolKind,
    },
    CurrentSynSymbol {
        ident: Ident,
        regional_token_idx: RegionalTokenIdx,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
    },
    FrameVarDecl {
        regional_token_idx: RegionalTokenIdx,
        ident: Ident,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        current_syn_symbol_kind: CurrentSynSymbolKind,
    },
    SelfType(RegionalTokenIdx),
    SelfValue(RegionalTokenIdx),
    Binary {
        lopd: SynExprIdx,
        opr: SynBinaryOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        ropd: SynExprIdx,
    },
    Be {
        src: SynExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: SynExprResult<BePatternSyndicate>,
    },
    Prefix {
        opr: SynPrefixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
        opd: SynExprIdx,
    },
    Suffix {
        opd: SynExprIdx,
        opr: SynSuffixOpr,
        opr_regional_token_idx: RegionalTokenIdx,
    },
    /// we shall need the exact type of `f` to disambiguate the following:
    /// - `f(x1, ..., xn)` can be interpreted in two ways:
    ///   - `f` is a curry function, `(x1, ..., xn)` is a tuple, this is an application
    ///   - `f` is a Ritchie function, `(x1, ..., xn)` is the list of arguments, this is a Ritchie function call
    ///
    /// - `f(x)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon `x`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with one element
    ///
    /// - `f(x,)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon one element tuple `(x,)`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with one element
    ///
    /// - `f()` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon unit `()`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with zero element
    ///
    /// - `f(,)` can be interpreted in two ways:
    ///   - `f` is a curry function, this is an application of `f` upon zero element tuple `(,)`
    ///   - `f` is a Ritchie function, this is a Ritchie function call with zero element
    FunctionApplicationOrCall {
        function: SynExprIdx,
        template_arguments: Option<SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// function type or trait
    Ritchie {
        ritchie_kind: RitchieKind,
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        lpar_token: LparRegionalToken,
        parameter_ty_items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
        light_arrow_token: Option<LightArrowRegionalToken>,
        /// it's guaranteed that `return_ty_expr` is some if and only if
        /// `light_arrow_token` is some
        return_ty_syn_expr_idx: Option<SynExprIdx>,
    },
    Closure {
        closure_kind_regional_token_idx: Option<RegionalTokenIdx>,
        lvert_regional_token_idx: RegionalTokenIdx,
        parameters: PunctuatedSmallList<
            ClosureParameterSyndicate,
            CommaRegionalToken,
            SynExprError,
            true,
            3,
        >,
        rvert_regional_token: RvertRegionalToken,
        /// in husky, `=` is needed after lambda return type to disambiguate `{`
        return_ty: Option<(LightArrowRegionalToken, SynExprIdx, EqRegionalToken)>,
        body: SynExprIdx,
    },
    FunctionCall {
        function: SynExprIdx,
        template_arguments: Option<SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCallListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    Field {
        owner: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    },
    MethodApplicationOrCall {
        self_argument: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        template_arguments: Option<SynTemplateArguments>,
        lpar_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    TemplateInstantiation {
        template: SynExprIdx,
        template_arguments: SynTemplateArguments,
    },
    // todo: implicit arguments
    ExplicitApplication {
        function_expr_idx: SynExprIdx,
        argument_expr_idx: SynExprIdx,
    },
    At {
        at_regional_token_idx: RegionalTokenIdx,
        place_label_regional_token: Option<PlaceLabelRegionalToken>,
    },
    Unit {
        lpar_regional_token_idx: RegionalTokenIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    Delimitered {
        lpar_regional_token_idx: RegionalTokenIdx,
        item: SynExprIdx,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    NewTuple {
        lpar_regional_token_idx: RegionalTokenIdx,
        /// guaranteed that items.len() > 0
        items: SmallVec<[SynCommaListItem; 4]>,
        rpar_regional_token_idx: RegionalTokenIdx,
    },
    /// there are two cases
    /// - index `$owner[$items]` where `$owner` can be indexed
    /// - application `$owner [$items]` where `$owner` is of type `List _ -> S`
    /// the cases are determined by whether `$owner` is of curry type
    IndexOrCompositionWithList {
        owner: SynExprIdx,
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    List {
        lbox_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    /// `[:]` means Slice
    /// `[:n]` means array as `[_;n]` in Rust
    /// `[:n1, n2, ...]` means multidimensional array
    BoxColonList {
        lbox_regional_token_idx: RegionalTokenIdx,
        colon_regional_token_idx: RegionalTokenIdx,
        items: SmallVec<[SynCommaListItem; 4]>,
        rbox_regional_token_idx: RegionalTokenIdx,
    },
    Block {
        stmts: SynStmtIdxRange,
    },
    NestedBlock {
        lcurl_regional_token_idx: RegionalTokenIdx,
        stmts: SynStmtIdxRange,
        rcurl_regional_token: NestedRcurlRegionalToken,
    },
    // todo: handle container
    EmptyHtmlTag {
        empty_html_bra_idx: RegionalTokenIdx,
        function_ident: IdentRegionalToken,
        arguments: IdentMap<SynHtmlArgumentExpr>,
        empty_html_ket: EmptyHtmlKetRegionalToken,
    },
    /// sorry is for comptime (say proof) terms
    Sorry {
        regional_token_idx: RegionalTokenIdx,
    },
    /// todo is for runtime terms
    Todo {
        regional_token_idx: RegionalTokenIdx,
    },
    Unreachable {
        regional_token_idx: RegionalTokenIdx,
    },
    Err(SynExprError),
}

impl From<IdentifiableEntityPathExpr> for SynExprData {
    fn from(expr: IdentifiableEntityPathExpr) -> Self {
        match expr {
            IdentifiableEntityPathExpr::Principal {
                path_expr_idx,
                opt_path,
            } => SynExprData::PrincipalEntityPath {
                path_expr_idx,
                opt_path,
            },
            IdentifiableEntityPathExpr::AssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            } => SynExprData::AssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
            },
        }
    }
}

pub type SynExprArena = Arena<SynExprData>;
pub type SynExprIdx = ArenaIdx<SynExprData>;
pub type SynExprIdxRange = ArenaIdxRange<SynExprData>;
pub type SynExprMap<V> = ArenaMap<SynExprData, V>;

#[derive(Debug, PartialEq, Eq)]
pub struct SynTemplateArguments {
    langle_regional_token_idx: RegionalTokenIdx,
    arguments: SmallVec<[SynCommaListItem; 4]>,
    rangle_regional_token_idx: RegionalTokenIdx,
}

impl SynTemplateArguments {
    pub(crate) fn new(
        langle_regional_token_idx: RegionalTokenIdx,
        arguments: SmallVec<[SynCommaListItem; 4]>,
        rangle_regional_token_idx: RegionalTokenIdx,
    ) -> Self {
        Self {
            langle_regional_token_idx,
            arguments,
            rangle_regional_token_idx,
        }
    }

    pub fn langle_regional_token_idx(&self) -> RegionalTokenIdx {
        self.langle_regional_token_idx
    }

    pub fn arguments(&self) -> &[SynCommaListItem] {
        &self.arguments
    }

    pub fn rangle_regional_token_idx(&self) -> RegionalTokenIdx {
        self.rangle_regional_token_idx
    }
}
