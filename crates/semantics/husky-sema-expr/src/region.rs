use husky_print_utils::p;
use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(db = SemaExprDb, jar = SemaExprJar, constructor = new_inner)]
pub struct SemaExprRegion {
    #[id]
    pub path: SynNodeRegionPath,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub data: SemaExprRegionData,
}

impl SemaExprRegion {
    pub(crate) fn new(
        path: SynNodeRegionPath,
        syn_expr_region: SynExprRegion,
        sema_expr_arena: SemaExprArena,
        sema_stmt_arena: SemaStmtArena,
        sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FlyTerm>>,
        symbol_tys: SymbolMap<SymbolType>,
        symbol_terms: SymbolMap<FlyTerm>,
        fluffy_term_region: FlyTermRegion,
        return_ty: Option<EthTerm>,
        self_ty: Option<EthTerm>,
        db: &::salsa::Db,
    ) -> Self {
        SemaExprRegion::new_inner(
            db,
            path,
            syn_expr_region,
            SemaExprRegionData {
                path,
                sema_expr_arena,
                sema_stmt_arena,
                sema_expr_roots,
                syn_pattern_expr_ty_infos: pattern_expr_ty_infos,
                syn_pattern_symbol_ty_infos: pattern_symbol_ty_infos,
                sema_expr_terms,
                symbol_tys,
                symbol_terms,
                fluffy_term_region,
                return_ty,
                self_ty,
            },
        )
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprRegionData {
    path: SynNodeRegionPath,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
    syn_pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    syn_pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FlyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FlyTerm>,
    fluffy_term_region: FlyTermRegion,
    return_ty: Option<EthTerm>,
    self_ty: Option<EthTerm>,
}

impl SemaExprRegionData {
    pub fn syn_root_to_sema_expr_idx(&self, syn_expr_root: SynExprIdx) -> SemaExprIdx {
        (self.sema_expr_roots[syn_expr_root].1).0
    }

    pub fn sema_expr_roots<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SemaExprIdx, SynExprRootKind)> + 'a {
        self.sema_expr_roots.iter().map(|&(_, root)| root)
    }

    pub fn sema_expr_arena(&self) -> SemaExprArenaRef {
        self.sema_expr_arena.arena_ref()
    }

    pub fn sema_expr_arena2(&self) -> &SemaExprArena {
        &self.sema_expr_arena
    }

    pub fn sema_stmt_arena(&self) -> SemaStmtArenaRef {
        self.sema_stmt_arena.arena_ref()
    }

    pub fn sema_expr_term(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<SemaExprTermResultRef<FlyTerm>> {
        Some(
            self.sema_expr_terms
                .get_value(sema_expr_idx)?
                .as_ref()
                .copied(),
        )
    }

    /// make sure the syn_expr_root is indeed a syn expr root
    pub fn syn_root_expr_term(
        &self,
        syn_expr_root: SynExprIdx,
    ) -> Option<SemaExprTermResultRef<FlyTerm>> {
        self.sema_expr_term(self.syn_root_to_sema_expr_idx(syn_expr_root))
    }

    pub fn fluffy_term_region(&self) -> &FlyTermRegion {
        &self.fluffy_term_region
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FlyTerm> {
        &self.symbol_terms
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }

    pub fn sema_expr_terms(&self) -> &VecPairMap<SemaExprIdx, SemaExprTermResult<FlyTerm>> {
        &self.sema_expr_terms
    }

    pub fn syn_pattern_expr_ty(
        &self,
        syn_pattern_expr_idx: idx_arena::ArenaIdx<SynPatternExprData>,
        db: &::salsa::Db,
    ) -> EthTerm {
        match self.syn_pattern_expr_ty_infos[syn_pattern_expr_idx].ty {
            Ok(ty_term) => match ty_term.base_resolved_inner(self.fluffy_term_region.terms()) {
                FlyTermBase::Ethereal(ty_term) => ty_term,
                FlyTermBase::Solid(_) => todo!(),
                FlyTermBase::Hollow(_) => todo!(),
                FlyTermBase::Place => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    pub fn return_ty(&self) -> Option<EthTerm> {
        self.return_ty
    }
}

#[salsa::tracked(jar = SemaExprJar)]
pub(crate) fn sema_expr_region(db: &::salsa::Db, syn_expr_region: SynExprRegion) -> SemaExprRegion {
    let mut engine = SemaExprEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct PatternExprTypeInfo {
    ty: PatternSemaExprResult<FlyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternSemaExprResult<FlyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FlyTerm, &PatternSemaExprError> {
        self.ty.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FlyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FlyTerm>) -> Self {
        Self { ty }
    }
}
