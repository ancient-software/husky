use crate::*;
use husky_vfs::Toolchain;

#[salsa::tracked(db = SynExprDb, jar = SynExprJar)]
pub struct SynExprRegion {
    #[return_ref]
    pub data: SynExprRegionData,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynExprRegionData {
    parent: Option<SynExprRegion>,
    path: RegionPath,
    expr_arena: SynExprArena,
    principal_item_path_expr_arena: SynPrincipalEntityPathExprArena,
    stmt_arena: SynStmtArena,
    pattern_expr_region: SynPatternExprRegion,
    symbol_region: SynSymbolRegion,
    syn_pattern_expr_roots: Vec<SynPatternExprRoot>,
    syn_expr_roots: Vec<SynExprRoot>,
    has_self_lifetime: bool,
    has_self_place: bool,
}

impl SynExprRegionData {
    pub(crate) fn new(
        parent: Option<SynExprRegion>,
        path: RegionPath,
        expr_arena: SynExprArena,
        item_path_expr_arena: SynPrincipalEntityPathExprArena,
        stmt_arena: SynStmtArena,
        pattern_expr_region: SynPatternExprRegion,
        symbol_region: SynSymbolRegion,
        syn_pattern_expr_roots: Vec<SynPatternExprRoot>,
        syn_expr_roots: Vec<SynExprRoot>,
        has_self_lifetime: bool,
        has_self_place: bool,
    ) -> Self {
        Self {
            parent,
            path,
            expr_arena,
            principal_item_path_expr_arena: item_path_expr_arena,
            stmt_arena,
            pattern_expr_region,
            symbol_region,
            syn_pattern_expr_roots,
            syn_expr_roots,
            has_self_lifetime,
            has_self_place,
        }
    }

    pub fn parent(&self) -> Option<SynExprRegion> {
        self.parent
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn path_ref(&self) -> &RegionPath {
        &self.path
    }

    pub fn expr_arena(&self) -> &SynExprArena {
        &self.expr_arena
    }

    pub fn principal_item_path_expr_arena(&self) -> &SynPrincipalEntityPathExprArena {
        &self.principal_item_path_expr_arena
    }

    pub fn pattern_expr_arena(&self) -> &SynPatternExprArena {
        self.pattern_expr_region.pattern_expr_arena()
    }

    pub fn stmt_arena(&self) -> &SynStmtArena {
        &self.stmt_arena
    }

    pub fn pattern_expr_region(&self) -> &SynPatternExprRegion {
        &self.pattern_expr_region
    }

    pub fn symbol_region(&self) -> &SynSymbolRegion {
        &self.symbol_region
    }

    pub fn syn_expr_roots(&self) -> &[SynExprRoot] {
        self.syn_expr_roots.as_ref()
    }

    pub fn return_ty(&self) -> Option<SynExprIdx> {
        self.syn_expr_roots.iter().find_map(|root| {
            (root.kind() == SynExprRootKind::ReturnType).then_some(root.syn_expr_idx())
        })
    }

    pub fn syn_pattern_expr_roots(&self) -> &[SynPatternExprRoot] {
        self.syn_pattern_expr_roots.as_ref()
    }

    pub fn self_ty(&self) -> Option<SynExprIdx> {
        todo!();
        // ad hoc
        // this will not work for derive any decl region\
        // self.roots
        //     .iter()
        //     .find_map(|root| (root.kind() == ExprRootKind::SelfType).then_some(root.expr_idx()))
    }

    pub fn has_self_lifetime(&self) -> bool {
        self.has_self_lifetime
    }

    pub fn has_self_place(&self) -> bool {
        self.has_self_place
    }
}

impl std::ops::Index<SynExprIdx> for SynExprRegionData {
    type Output = SynExprData;

    fn index(&self, index: SynExprIdx) -> &Self::Output {
        &self.expr_arena[index]
    }
}

impl std::ops::Index<SynStmtIdx> for SynExprRegionData {
    type Output = SynStmtData;

    fn index(&self, index: SynStmtIdx) -> &Self::Output {
        &self.stmt_arena[index]
    }
}
impl std::ops::Index<CurrentSynSymbolIdx> for SynExprRegionData {
    type Output = CurrentSynSymbol;

    fn index(&self, index: CurrentSynSymbolIdx) -> &Self::Output {
        &self.symbol_region[index]
    }
}
impl std::ops::Index<SynPatternSymbolIdx> for SynExprRegionData {
    type Output = SynPatternSymbol;

    fn index(&self, index: SynPatternSymbolIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}
impl std::ops::Index<SynPatternExprIdx> for SynExprRegionData {
    type Output = SynPatternExpr;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_region[index]
    }
}

impl SynExprRegion {
    pub fn toolchain(self, db: &dyn SynExprDb) -> Toolchain {
        // ad hoc
        match self.data(db).path {
            RegionPath::Snippet(module_path) => module_path.toolchain(db),
            RegionPath::Decl(syn_node_path) | RegionPath::Defn(syn_node_path) => {
                syn_node_path.toolchain(db)
            }
        }
    }
}
