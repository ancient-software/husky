use super::*;
use crate::{
    division::{
        VdSynDivisionArenaRef, VdSynDivisionChild, VdSynDivisionIdx, VdSynDivisionIdxRange,
        VdSynDivisionMap,
    },
    stmt::{VdSynStmtArenaRef, VdSynStmtData, VdSynStmtIdx, VdSynStmtIdxRange, VdSynStmtMap},
};
use latex_vfs::path::LxFilePath;
use smallvec::SmallVec;
use visored_item_path::module::{VdModulePath, VdModulePathRegistry};

pub struct VdSynExprEntityTreeBuilder<'a> {
    db: &'a ::salsa::Db,
    file_path: LxFilePath,
    stmt_arena: VdSynStmtArenaRef<'a>,
    division_arena: VdSynDivisionArenaRef<'a>,
    stmt_module_path_node_map: VdSynStmtMap<VdModulePath>,
    division_module_path_node_map: VdSynDivisionMap<VdModulePath>,
}

impl VdSynExprEntityTreeNode {
    pub fn module_path(&self) -> VdModulePath {
        self.module_path
    }

    pub fn children(&self) -> &[VdModulePath] {
        &self.children
    }
}

impl<'a> VdSynExprEntityTreeBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        file_path: LxFilePath,
        stmt_arena: VdSynStmtArenaRef<'a>,
        division_arena: VdSynDivisionArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            file_path,
            stmt_arena,
            division_arena,
            stmt_module_path_node_map: VdSynStmtMap::new2(stmt_arena),
            division_module_path_node_map: VdSynDivisionMap::new2(division_arena),
        }
    }
}

impl<'a> VdSynExprEntityTreeBuilder<'a> {
    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn file_path(&self) -> LxFilePath {
        self.file_path
    }
}

impl<'a> VdSynExprEntityTreeBuilder<'a> {
    pub fn build_root_stmts(&mut self, stmts: VdSynStmtIdxRange) -> VdSynExprEntityTreeNode {
        let module_path = VdModulePath::new_root(self.db, self.file_path);
        let mut registry = VdModulePathRegistry::new(module_path);
        let children = self.build_stmts(stmts, &mut registry);
        VdSynExprEntityTreeNode {
            module_path,
            children,
        }
    }

    fn build_divisions(
        &mut self,
        divisions: VdSynDivisionIdxRange,
        registry: &mut VdModulePathRegistry,
    ) {
        todo!()
    }

    fn build_division(
        &mut self,
        division: VdSynDivisionIdx,
        registry: &mut VdModulePathRegistry,
    ) -> VdModulePath {
        let node = self.calc_division(division, registry);
        let module_path = node.module_path();
        self.division_module_path_node_map
            .insert_new(division, module_path);
        module_path
    }

    fn calc_division(
        &mut self,
        division: VdSynDivisionIdx,
        registry: &mut VdModulePathRegistry,
    ) -> VdSynExprEntityTreeNode {
        let division_arena = self.division_arena;
        let division_data = &division_arena[division];
        let module_path = registry.issue_new_division(division_data.kind(), self.db);
        let mut division_registry = VdModulePathRegistry::new(module_path);
        let children = division_data
            .children()
            .iter()
            .map(|&child| match child {
                VdSynDivisionChild::Division(division) => {
                    self.build_division(division, &mut division_registry)
                }
                VdSynDivisionChild::Stmt(stmt) => self.build_stmt(stmt, &mut division_registry),
            })
            .collect();
        VdSynExprEntityTreeNode {
            module_path,
            children,
        }
    }

    fn build_stmts(
        &mut self,
        stmts: VdSynStmtIdxRange,
        registry: &mut VdModulePathRegistry,
    ) -> Vec<VdModulePath> {
        stmts
            .into_iter()
            .map(|stmt| self.build_stmt(stmt, registry))
            .collect()
    }

    fn build_stmt(
        &mut self,
        stmt: VdSynStmtIdx,
        registry: &mut VdModulePathRegistry,
    ) -> VdModulePath {
        let node = self.calc_stmt(stmt, registry);
        let module_path = node.module_path();
        self.stmt_module_path_node_map.insert_new(stmt, module_path);
        module_path
    }

    fn calc_stmt(
        &mut self,
        stmt: VdSynStmtIdx,
        registry: &mut VdModulePathRegistry,
    ) -> VdSynExprEntityTreeNode {
        let stmt_arena = self.stmt_arena;
        let (module_path, children) = match stmt_arena[stmt] {
            VdSynStmtData::Paragraph(_) => {
                let module_path = registry.issue_new_paragraph(self.db);
                (module_path, vec![])
            }
            VdSynStmtData::Environment {
                environment_signature,
                stmts,
            } => todo!(),
        };
        VdSynExprEntityTreeNode {
            module_path,
            children,
        }
    }

    pub(super) fn finish(self) -> (VdSynStmtMap<VdModulePath>, VdSynDivisionMap<VdModulePath>) {
        (
            self.stmt_module_path_node_map,
            self.division_module_path_node_map,
        )
    }
}
