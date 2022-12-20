use crate::*;

use husky_ast::AstDb;


use husky_entity_card::{EntityCard};
use husky_entity_path::EntityPath;





use husky_toolchain_infer::ToolchainInferDb;
use husky_vfs::*;

use salsa::DbWithJar;


pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + AstDb + ToolchainInferDb {
    fn entity_absolute_path(
        &self,
        entity_path: EntityPath,
    ) -> &EntityTreeResult<AbsoluteEntityPath>;
    fn entity_tree_sheet(&self, module: EntityPath) -> &VfsResult<EntityTreeSheet>;
    fn entity_card(&self, entity_path: EntityPath) -> &EntityTreeResult<EntityCard>;
    fn is_absolute(&self, entity_path: EntityPath) -> EntityTreeResult<bool> {
        Ok(self.entity_absolute_path(entity_path).as_ref()?.path() == entity_path)
    }
    fn submodules(&self, entity_path: EntityPath) -> &VfsResult<Vec<EntityPath>>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &VfsResult<Vec<EntityPath>>;
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + AstDb + ToolchainInferDb,
{
    fn entity_absolute_path(
        &self,
        entity_path: EntityPath,
    ) -> &EntityTreeResult<AbsoluteEntityPath> {
        absolute_entity_path(self, entity_path)
    }

    fn entity_card(&self, entity_path: EntityPath) -> &EntityTreeResult<EntityCard> {
        entity_card(self, entity_path)
    }

    fn entity_tree_sheet(&self, module: EntityPath) -> &VfsResult<EntityTreeSheet> {
        entity_tree_sheet(self, module)
    }

    fn submodules(&self, entity_path: EntityPath) -> &VfsResult<Vec<EntityPath>> {
        submodules(self, entity_path)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &VfsResult<Vec<EntityPath>> {
        all_modules_within_crate(self, crate_path)
    }
}
